use std::{collections::HashMap, sync::Arc};

use actix::{Actor, Addr, AsyncContext, Handler};
use anyhow::{anyhow, Result};
use serde_json::{Map, Value};

use super::{
    actor_step_context::ActorStepContext,
    job_worker_actor::{JobCompletedMessage, JobWorkerActor},
};
use crate::definition::{process_definition::ProcessDefinition, step::StepInputRequest};

#[derive(Debug, Clone)]
pub enum StepExecutionStatus {
    Started,
    Waiting,
    Completed,
}

#[derive(Debug, Clone)]
pub struct StepState {
    pub step_id: String,
    pub status: StepExecutionStatus,
    pub inputs: Map<String, Value>,
    pub outputs: Map<String, Value>,
}

impl StepState {
    pub fn new(step_id: String) -> Self {
        Self {
            step_id,
            status: StepExecutionStatus::Started,
            inputs: Map::default(),
            outputs: Map::default(),
        }
    }
}

pub struct ProcessActor {
    job_worker: Addr<JobWorkerActor>,
    process_definition: Arc<ProcessDefinition>,
    process_inputs: Map<String, Value>,
    jobs: HashMap<String, String>,
    steps: HashMap<String, StepState>,
}

impl ProcessActor {
    pub fn new(
        job_worker: Addr<JobWorkerActor>,
        process_definition: Arc<ProcessDefinition>,
        process_inputs: Map<String, Value>,
    ) -> Self {
        let jobs = HashMap::default();

        Self {
            jobs,
            job_worker,
            process_inputs,
            process_definition,
            steps: HashMap::default(),
        }
    }

    fn resolve_input_requests(
        &self,
        input_requests: &Vec<StepInputRequest>,
    ) -> Result<Map<String, Value>> {
        let mut inputs = Map::default();

        for input_request in input_requests {
            let value = self
                .steps
                .get(&input_request.from)
                .ok_or_else(|| anyhow::anyhow!("Step not found"))?
                .outputs
                .get(&input_request.output)
                .ok_or_else(|| anyhow::anyhow!("Output not found"))?;

            inputs.insert(input_request.name.clone(), value.clone());
        }

        Ok(inputs)
    }

    fn start_step(&mut self, step_id: String) -> Result<()> {
        let step = self
            .process_definition
            .get_step(&step_id)
            .ok_or_else(|| anyhow::anyhow!("Step not found"))?;

        let input_requests = step.get_input_requests();
        let inputs = self.resolve_input_requests(&input_requests)?;

        let step_state = self
            .steps
            .entry(step_id.clone())
            .or_insert(StepState::new(step_id.clone()));

        step_state.inputs.extend(inputs);

        let ctx = ActorStepContext::new(self.job_worker.clone(), step_state.inputs.clone());
        let result = step.start(&ctx)?;

        match result {
            crate::definition::step::StepResult::AsyncJob(job_id) => {
                step_state.status = StepExecutionStatus::Waiting;
                self.jobs.insert(job_id, step_id);
            }
            crate::definition::step::StepResult::Completed(outputs) => {
                step_state.status = StepExecutionStatus::Completed;
                step_state.outputs.extend(outputs);
                self.execute_next_steps(&step.id())?;
            }
        }

        Ok(())
    }

    fn execute_next_steps(&mut self, id: &str) -> Result<()> {
        let next_steps = self.process_definition.get_next(id);

        if next_steps.is_none() {
            return Ok(());
        }

        let step_list = next_steps.unwrap().clone();

        for next_step_id in step_list {
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

        self.start_step(self.process_definition.get_start_step_id())
            .unwrap();

        self.job_worker
            .do_send(crate::actors::job_worker_actor::AddCompletionSubscriber(
                ctx.address().recipient(),
            ));
    }
}

impl Handler<JobCompletedMessage> for ProcessActor {
    type Result = Result<()>;

    fn handle(&mut self, msg: JobCompletedMessage, _ctx: &mut Self::Context) -> Self::Result {
        let step_id = self
            .jobs
            .remove(&msg.job_id)
            .ok_or_else(|| anyhow::anyhow!("Job {} not found", msg.job_id))?;

        let step = self
            .process_definition
            .get_step(&step_id)
            .ok_or_else(|| anyhow::anyhow!("Step {} not found", step_id))?;

        let ctx =
            ActorStepContext::new(self.job_worker.clone(), self.steps[&step_id].inputs.clone());
        let result = step.end(&ctx)?;

        match result {
            crate::definition::step::StepResult::AsyncJob(_) => {
                return Err(anyhow!("Async job not supported in end"));
            }
            crate::definition::step::StepResult::Completed(outputs) => {
                let step_state = self.steps.get_mut(&step_id).unwrap();
                step_state.status = StepExecutionStatus::Completed;
                step_state.outputs.extend(outputs);

                self.execute_next_steps(&step.id())?;
            }
        }

        Ok(())
    }
}
