use std::sync::Arc;

use super::process_definition::ProcessDefinition;

pub struct ProcessValidator {
    process_definition: Arc<ProcessDefinition>,
    visited_edges: Vec<String>,
}

impl ProcessValidator {
    pub fn new(process_definition: Arc<ProcessDefinition>) -> Self {
        ProcessValidator {
            process_definition,
            visited_edges: vec![],
        }
    }

    pub fn validate(&mut self) -> bool {
        let end_steps = self.process_definition.get_end_step_ids().clone();

        for end_step in end_steps {
            if !self.visit(&end_step) {
                return false;
            }
        }

        true
    }

    fn visit(&mut self, step_id: &str) -> bool {
        if self.visited_edges.contains(&step_id.to_string()) {
            return false;
        }

        self.visited_edges.push(step_id.to_string());

        let next_steps = self.process_definition.get_next(step_id);

        if next_steps.is_none() {
            return true;
        }

        // let next_steps = next_steps.unwrap().clone();

        // for next_step in next_steps {
        //     if !self.visit(&next_step) {
        //         return false;
        //     }
        // }

        true
    }
}
