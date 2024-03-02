use std::collections::HashMap;

use super::step::Step;

pub struct ProcessDefinition {
    start_step_id: String,
    steps: HashMap<String, Box<dyn Step>>,
    flow: HashMap<String, Vec<String>>,
}

impl ProcessDefinition {
    pub fn new(
        steps: HashMap<String, Box<dyn Step>>,
        flow: HashMap<String, Vec<String>>,
        start_step_id: String,
    ) -> Self {
        Self {
            steps,
            flow,
            start_step_id,
        }
    }

    pub fn get_start_step_id(&self) -> String {
        self.start_step_id.clone()
    }

    pub fn get_step(&self, id: &str) -> Option<&Box<dyn Step>> {
        self.steps.get(id)
    }

    pub fn get_next(&self, id: &str) -> Option<&Vec<String>> {
        self.flow.get(id)
    }
}
