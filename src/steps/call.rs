use crate::definition::step::{Step, StepInputRequest, StepResult};

pub struct CallStep {
    pub id: String,
    pub process: String,
    pub inputs: Vec<StepInputRequest>,
}

impl CallStep {
    pub fn new(id: String, process: String, inputs: Vec<StepInputRequest>) -> Self {
        Self {
            id,
            process,
            inputs,
        }
    }
}

impl Step for CallStep {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn get_input_requests(&self) -> Vec<StepInputRequest> {
        self.inputs.clone()
    }

    fn start(
        &self,
        ctx: &dyn crate::definition::step::ManageStep,
    ) -> anyhow::Result<crate::definition::step::StepResult> {
        let job_id = ctx.start_process(self.process.clone(), ctx.get_inputs().clone())?;

        Ok(StepResult::AsyncJob(job_id))
    }
    
    fn get_type(&self) -> crate::definition::step::StepType {
        crate::definition::step::StepType::CallStep
    }
}
