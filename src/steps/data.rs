use serde_json::{Map, Value};

use crate::definition::step::Step;

#[derive(Debug, Clone)]
pub struct DataStep {
    pub id: String,
    pub value: Value,
}

impl DataStep {
    pub fn new(id: String, value: Value) -> Self {
        Self { id, value }
    }
}

impl Step for DataStep {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn start(
        &self,
        _ctx: &dyn crate::definition::step::ManageStep,
    ) -> anyhow::Result<crate::definition::step::StepResult> {
        let outputs = {
            let mut outputs = Map::default();
            outputs.insert("value".to_string(), self.value.clone());
            outputs
        };

        Ok(crate::definition::step::StepResult::Completed(outputs))
    }
}
