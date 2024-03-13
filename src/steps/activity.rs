use anyhow::Result;

use crate::definition::step::{ManageStep, Step, StepInputRequest, StepResult};

#[derive(Debug, Clone)]
pub struct ActivityStep {
    id: String,
    name: String,
    job: String,
    input_schema: String,
    output_schema: String,
    inputs: Vec<StepInputRequest>,
}

impl ActivityStep {
    pub fn new(
        id: String,
        name: String,
        input_schema: String,
        output_schema: String,
        job: String,
        inputs: Vec<StepInputRequest>,
    ) -> Self {
        Self {
            id,
            name,
            input_schema,
            output_schema,
            job,
            inputs,
        }
    }
}

impl Step for ActivityStep {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn input_schema(&self) -> Option<String> {
        Some(self.input_schema.clone())
    }

    fn output_schema(&self) -> Option<String> {
        Some(self.output_schema.clone())
    }

    fn get_input_requests(&self) -> Vec<StepInputRequest> {
        self.inputs.clone()
    }

    fn start(&self, ctx: &dyn ManageStep) -> Result<StepResult> {
        let job_id = ctx.add_job(self.job.clone());

        Ok(StepResult::AsyncJob(job_id))
    }
}
