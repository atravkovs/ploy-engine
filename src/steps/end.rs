use crate::definition::step::{Step, StepInputRequest};

pub struct EndStep {
    pub id: String,
    pub inputs: Vec<StepInputRequest>,
}

impl EndStep {
    pub fn new(id: String, inputs: Vec<StepInputRequest>) -> Self {
        Self { id, inputs }
    }
}

impl Step for EndStep {
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
        Ok(crate::definition::step::StepResult::ProcessEnded(
            ctx.get_inputs().clone(),
        ))
    }

    fn get_type(&self) -> crate::definition::step::StepType {
        crate::definition::step::StepType::EndStep
    }
}
