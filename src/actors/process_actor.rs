use std::{collections::HashMap, sync::Arc};

use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, Message};
use anyhow::{anyhow, Result};
use log::{info, warn};
use serde_json::{Map, Value};

use super::{
    actor_step_context::ActorStepContext,
    engine_actor::EngineActor,
    job_worker_actor::{JobCompletedMessage, JobWorkerActor},
};
use crate::{
    actors::engine_actor,
    definition::{
        process_definition::ProcessDefinition,
        step::{StepExecutionStatus, StepInputRequest, StepState},
    },
};

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct EndProcessMessage {}

pub struct ProcessActor {
    id: String,
    process_engine: Addr<EngineActor>,
    job_worker: Addr<JobWorkerActor>,
    process_definition: Arc<ProcessDefinition>,
    process_inputs: Map<String, Value>,
    jobs: HashMap<String, String>,
    steps: HashMap<String, StepState>,
}

impl ProcessActor {
    pub fn new(
        id: String,
        process_engine: Addr<EngineActor>,
        job_worker: Addr<JobWorkerActor>,
        process_definition: Arc<ProcessDefinition>,
        process_inputs: Map<String, Value>,
    ) -> Self {
        let jobs = HashMap::default();

        Self {
            id,
            jobs,
            job_worker,
            process_engine,
            process_inputs,
            process_definition,
            steps: HashMap::default(),
        }
    }

    fn resolve_input_requests(
        &mut self,
        input_requests: &Vec<StepInputRequest>,
    ) -> Result<Map<String, Value>> {
        let mut inputs = Map::default();

        for input_request in input_requests {
            let from_step_state: Option<&StepState> = {
                let state = self.steps.get(&input_request.from);

                if state.is_some() {
                    state
                } else {
                    self.start_step(input_request.from.clone())
                        .expect("Start step failed");

                    self.steps.get(&input_request.from)
                }
            };

            let outputs = from_step_state
                .expect("Step should have been executed first")
                .outputs
                .clone();

            inputs.insert(
                input_request.name.clone(),
                outputs
                    .get(input_request.output.as_str())
                    .expect(
                        format!(
                            "Step {} should have outputted {} value",
                            input_request.from, input_request.output
                        )
                        .as_str(),
                    )
                    .clone(),
            );
        }

        Ok(inputs)
    }

    fn validate_map(map: &Map<String, Value>, schema_name: &str) -> Result<()> {
        let schema_contents =
            std::fs::read_to_string(format!("data/schemas/{schema_name}.json")).unwrap();

        let schema: Value = serde_json::from_str(&schema_contents).unwrap();

        let value = Value::Object(map.clone());

        let compiled = jsonschema::JSONSchema::compile(&schema).expect("A valid schema");

        let result = compiled.validate(&value);

        if let Err(errors) = result {
            let err_message = errors
                .map(|err| err.to_string())
                .collect::<Vec<String>>()
                .join("\n");

            warn!("Validation failed: {}", err_message);

            Err(anyhow!(err_message))
        } else {
            Ok(())
        }
    }

    fn start_step(&mut self, step_id: String) -> Result<()> {
        let input_requests = self
            .process_definition
            .get_step(&step_id)
            .ok_or_else(|| anyhow::anyhow!("Step not found"))?
            .get_input_requests();

        let inputs = self.resolve_input_requests(&input_requests)?;

        let step = self
            .process_definition
            .get_step(&step_id)
            .ok_or_else(|| anyhow::anyhow!("Step not found"))?;

        if let Some(input_schema) = step.input_schema() {
            Self::validate_map(&inputs, &input_schema)?;
        }

        let step_state = self
            .steps
            .entry(step_id.clone())
            .or_insert(StepState::new(step_id.clone()));

        step_state.inputs.extend(inputs);

        info!("Starting step: {}", step_id);

        let ctx = ActorStepContext::new(
            self.id.clone(),
            self.process_engine.clone(),
            self.job_worker.clone(),
            step_state.inputs.clone(),
        );
        let result = step.start(&ctx)?;

        match result {
            crate::definition::step::StepResult::AsyncJob(job_id) => {
                step_state.status = StepExecutionStatus::Waiting;
                self.jobs.insert(job_id, step_id);
            }
            crate::definition::step::StepResult::Completed(outputs) => {
                self.complete_step(&step_id, outputs)?;
            }
            crate::definition::step::StepResult::ProcessEnded(outputs) => {
                self.process_engine
                    .do_send(engine_actor::EndProcessMessage {
                        process_id: self.id.clone(),
                        outputs,
                    });
            }
        }

        Ok(())
    }

    fn complete_step(&mut self, step_id: &str, outputs: Map<String, Value>) -> Result<()> {
        let step = self
            .process_definition
            .get_step(step_id)
            .ok_or_else(|| anyhow::anyhow!("Step not found"))?;

        let step_state = self
            .steps
            .get_mut(step_id)
            .ok_or_else(|| anyhow::anyhow!("Step state not found"))?;

        step_state.status = StepExecutionStatus::Completed;
        step_state.outputs.extend(outputs);

        if let Some(output_schema) = step.output_schema() {
            Self::validate_map(&step_state.outputs, &output_schema)?;
        }

        self.execute_next_steps(step_id)?;

        Ok(())
    }

    fn get_actor_step_context(&self, step_id: &str) -> Result<ActorStepContext> {
        let step_state = self
            .steps
            .get(step_id)
            .ok_or_else(|| anyhow::anyhow!("Step state not found"))?;

        Ok(ActorStepContext::new(
            self.id.clone(),
            self.process_engine.clone(),
            self.job_worker.clone(),
            step_state.inputs.clone(),
        ))
    }

    fn execute_next_steps(&mut self, step_id: &str) -> Result<()> {
        let step = self
            .process_definition
            .get_step(step_id)
            .ok_or_else(|| anyhow::anyhow!("Step not found"))?;

        let empty_steps = Vec::default();

        let next_steps = self
            .process_definition
            .get_next(step_id)
            .unwrap_or(&empty_steps);

        let ctx = self.get_actor_step_context(step_id)?;
        let next_steps = step.get_next_steps(&ctx, next_steps);

        for next_step_id in next_steps {
            self.start_step(next_step_id.clone())?;
        }

        Ok(())
    }
}

impl Actor for ProcessActor {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let mut start_step_state = StepState::new(self.process_definition.get_start_step_id());
        start_step_state.inputs.extend(self.process_inputs.clone());

        self.steps.insert(
            self.process_definition.get_start_step_id(),
            start_step_state,
        );

        self.job_worker
            .do_send(crate::actors::job_worker_actor::AddCompletionSubscriber(
                ctx.address().recipient(),
            ));

        if let Err(err) = self.start_step(self.process_definition.get_start_step_id()) {
            ctx.stop();
            log::error!("Failed to start process: {}", err);
        }
    }
}

impl Handler<JobCompletedMessage> for ProcessActor {
    type Result = Result<()>;

    fn handle(&mut self, msg: JobCompletedMessage, _ctx: &mut Self::Context) -> Self::Result {
        let step_id = self
            .jobs
            .remove(&msg.job_id)
            .ok_or_else(|| anyhow::anyhow!("Job {} not found", msg.job_id))?;

        self.complete_step(&step_id, msg.outputs)?;

        Ok(())
    }
}

impl Handler<EndProcessMessage> for ProcessActor {
    type Result = Result<()>;

    fn handle(&mut self, _msg: EndProcessMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
        Ok(())
    }
}
