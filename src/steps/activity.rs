use anyhow::Result;

use crate::definition::step::{ManageStep, Step, StepInputRequest, StepResult};

#[derive(Debug, Clone)]
pub struct ActivityStep {
    id: String,
    name: String,
    job: String,
    inputs: Vec<StepInputRequest>,
}

impl ActivityStep {
    pub fn new(id: String, name: String, job: String, inputs: Vec<StepInputRequest>) -> Self {
        Self {
            id,
            name,
            job,
            inputs,
        }
    }
}

impl Step for ActivityStep {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn get_input_requests(&self) -> Vec<StepInputRequest> {
        self.inputs.clone()
    }

    fn start(&self, ctx: &dyn ManageStep) -> Result<StepResult> {
        let job_id = ctx.add_job(self.job.clone());

        Ok(StepResult::AsyncJob(job_id))
    }
}
