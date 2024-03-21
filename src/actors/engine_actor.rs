use std::{collections::HashMap, sync::Arc};

use actix::{Actor, Addr, ArbiterHandle, AsyncContext, Handler, Message};
use anyhow::{anyhow, Result};
use log::info;
use serde_json::{Map, Value};

use crate::{
    actors::job_worker_actor::JobCompletedMessage,
    definition::process_definition::ProcessDefinition,
};

use super::{
    job_worker_actor::JobWorkerActor,
    process_actor::ProcessActor,
    process_context::{ProcessContext, ProcessState},
};

#[derive(Message)]
#[rtype(result = "anyhow::Result<String>")]
pub struct StartProcessMessage {
    pub root_process_id: Option<String>,
    pub job_id: Option<String>,
    pub process_name: String,
    pub inputs: Map<String, Value>,
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<ProcessContext>")]
pub struct GetProcessMessage {
    pub process_id: String,
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<bool>")]
pub struct ValidateProcessMessage {
    pub process_name: String,
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct EndProcessMessage {
    pub process_id: String,
    pub outputs: Map<String, Value>,
}

pub struct EngineActor {
    arbiter: ArbiterHandle,
    processes: HashMap<String, ProcessContext>,
    // sub_process_id -> (job_id, root_process_id)
    pending_job: HashMap<String, (String, String)>,
    process_definitions: HashMap<String, Arc<ProcessDefinition>>,
    job_worker: Addr<JobWorkerActor>,
}

impl EngineActor {
    pub fn new(arbiter: ArbiterHandle, job_worker: Addr<JobWorkerActor>) -> Self {
        let processes = HashMap::default();
        let process_definitions = HashMap::default();
        let pending_job = HashMap::default();

        Self {
            arbiter,
            processes,
            job_worker,
            pending_job,
            process_definitions,
        }
    }

    pub fn start_process(
        &mut self,
        process_name: &str,
        process_inputs: Map<String, Value>,
        ctx: &mut actix::Context<Self>,
    ) -> Result<String> {
        let process_definition = self.get_or_import_process_definition(process_name)?;

        let job_worker_actor = self.job_worker.clone();

        let my_addr = ctx.address();

        let process_id = uuid::Uuid::new_v4().to_string();
        let process_id_mv = process_id.clone();
        let process_actor_addr = ProcessActor::start_in_arbiter(&self.arbiter, |_ctx| {
            ProcessActor::new(
                process_id_mv,
                my_addr,
                job_worker_actor,
                process_definition,
                process_inputs,
            )
        });

        let process_context = ProcessContext::new(process_id.clone(), process_actor_addr);

        info!("Process started: {:#?}", process_context);

        self.processes.insert(process_id.clone(), process_context);

        Ok(process_id)
    }

    pub fn get_process(&self, process_id: &str) -> Result<&ProcessContext> {
        self.processes
            .get(process_id)
            .ok_or_else(|| anyhow!("Process not found"))
    }

    pub fn get_process_mut(&mut self, process_id: &str) -> Result<&mut ProcessContext> {
        self.processes
            .get_mut(process_id)
            .ok_or_else(|| anyhow!("Process not found"))
    }

    pub fn get_or_import_process_definition(
        &mut self,
        process_name: &str,
    ) -> Result<Arc<ProcessDefinition>> {
        let process_definition = self.process_definitions.get(process_name);

        if process_definition.is_none() {
            let process_definition = Self::import_process_definition(process_name)?;
            self.process_definitions
                .insert(process_name.to_string(), process_definition.clone());

            return Ok(process_definition);
        }

        Ok(process_definition
            .expect("Process definition is present")
            .clone())
    }

    pub fn get_process_definition(&self, process_name: &str) -> Result<Arc<ProcessDefinition>> {
        self.process_definitions
            .get(process_name)
            .cloned()
            .ok_or_else(|| anyhow!("Process definition not found"))
    }

    fn import_process_definition(process_name: &str) -> Result<Arc<ProcessDefinition>> {
        let file_name = format!("data/{process_name}.ploy");
        let file_contents = std::fs::read_to_string(file_name)?;

        let process_definition = Arc::new(crate::definition::parser::parse_xml(&file_contents)?);

        Ok(process_definition)
    }
}

impl Actor for EngineActor {
    type Context = actix::Context<Self>;
}

impl Handler<StartProcessMessage> for EngineActor {
    type Result = Result<String>;

    fn handle(&mut self, msg: StartProcessMessage, ctx: &mut Self::Context) -> Self::Result {
        let process_id = self.start_process(&msg.process_name, msg.inputs, ctx)?;

        if msg.job_id.is_some() && msg.root_process_id.is_some() {
            self.pending_job.insert(
                process_id.clone(),
                (msg.job_id.unwrap(), msg.root_process_id.unwrap()),
            );
        }

        Ok(process_id)
    }
}

impl Handler<EndProcessMessage> for EngineActor {
    type Result = Result<()>;

    fn handle(&mut self, msg: EndProcessMessage, _ctx: &mut Self::Context) -> Self::Result {
        {
            let process = self.get_process_mut(&msg.process_id)?;
            process.outputs = Some(msg.outputs);

            process
                .process_addr
                .as_ref()
                .expect("Process is alive before ending")
                .do_send(crate::actors::process_actor::EndProcessMessage {});

            process.process_addr = None;
            process.state = ProcessState::Completed;

            info!("Process completed: {:#?}", process);

            process
        };

        if let Some((job_id, root_process_id)) = self.pending_job.remove(&msg.process_id) {
            let process = self.get_process(&msg.process_id)?;
            let root_process = self.get_process(&root_process_id)?;

            root_process
                .process_addr
                .as_ref()
                .expect("Root process is alive before call activity ending")
                .do_send(JobCompletedMessage {
                    job_id,
                    outputs: process
                        .outputs
                        .as_ref()
                        .expect("Outputs are present")
                        .clone(),
                });
        }

        Ok(())
    }
}

impl Handler<GetProcessMessage> for EngineActor {
    type Result = Result<ProcessContext>;

    fn handle(&mut self, msg: GetProcessMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.get_process(&msg.process_id).map(|p| p.clone())
    }
}

impl Handler<ValidateProcessMessage> for EngineActor {
    type Result = Result<bool>;

    fn handle(&mut self, msg: ValidateProcessMessage, _ctx: &mut Self::Context) -> Self::Result {
        let process_definition = Self::import_process_definition(&msg.process_name)?;

        let process_validator =
            crate::definition::validator::ProcessValidator::new(process_definition);

        Ok(process_validator.validate())
    }
}
