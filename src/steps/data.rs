use serde_json::{Map, Value};

use crate::definition::step::Step;

#[derive(Debug, Clone)]
pub struct DataStep {
    pub id: String,
    pub outputs: Map<String, Value>,
}

impl DataStep {
    pub fn new(id: String, outputs: Map<String, Value>) -> Self {
        Self { id, outputs }
    }
}

impl Step for DataStep {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn get_output(
        &self,
        name: &str,
        _state: Option<&crate::definition::step::StepState>,
    ) -> serde_json::Value {
        self.outputs
            .get(name)
            .expect(&format!("Output with {} not found", name))
            .clone()
    }
}
