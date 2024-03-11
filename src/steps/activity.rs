use anyhow::Result;
use serde_json::Value;

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

    fn get_output(&self, name: &str, state: Option<&crate::definition::step::StepState>) -> Value {
        state
            .expect(format!("Step {} has to be executed first", self.id).as_str())
            .outputs
            .get(name)
            .expect(format!("Output {} not found", name).as_str())
            .clone()
    }

    fn get_input_requests(&self) -> Vec<StepInputRequest> {
        self.inputs.clone()
    }

    fn start(&self, ctx: &dyn ManageStep) -> Result<StepResult> {
        let job_id = ctx.add_job(self.job.clone());

        Ok(StepResult::AsyncJob(job_id))
    }

    fn is_async(&self) -> bool {
        true
    }
}
