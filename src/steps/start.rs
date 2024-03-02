use crate::definition::step::Step;

pub struct StartStep {
    pub id: String,
}

impl StartStep {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl Step for StartStep {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn start(
        &self,
        ctx: &dyn crate::definition::step::ManageStep,
    ) -> anyhow::Result<crate::definition::step::StepResult> {
        Ok(crate::definition::step::StepResult::Completed(
            ctx.get_inputs().clone(),
        ))
    }
}
