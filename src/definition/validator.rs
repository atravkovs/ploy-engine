use std::{collections::HashMap, sync::Arc};

use log::warn;

use super::process_definition::ProcessDefinition;

pub struct ProcessValidator {
    process_definition: Arc<ProcessDefinition>,
}

impl ProcessValidator {
    pub fn new(process_definition: Arc<ProcessDefinition>) -> Self {
        ProcessValidator { process_definition }
    }

    pub fn validate(&self) -> bool {
        let start_step = self.process_definition.get_start_step_id();

        let mut stack = HashMap::default();
        let mut visited = HashMap::default();

        self.validate_internal(&mut visited, &mut stack, &start_step)
    }

    fn validate_internal(
        &self,
        visited: &mut HashMap<String, bool>,
        stack: &mut HashMap<String, bool>,
        step_id: &str,
    ) -> bool {
        if stack
            .get(&step_id.to_string())
            .is_some_and(|item| item == &true)
        {
            warn!("Cycle detected: {}", step_id);
            return false;
        }

        if visited
            .get(&step_id.to_string())
            .is_some_and(|item| item == &true)
        {
            return true;
        }

        stack.insert(step_id.to_string(), true);
        visited.insert(step_id.to_string(), true);

        let next_steps = self.process_definition.get_next(step_id);

        if next_steps.is_none() || next_steps.unwrap().is_empty() {
            let step = self.process_definition.get_step(step_id).unwrap();

            if !step.get_type().is_end() {
                warn!("Last process step is not End: {}", step_id);
                return false;
            }

            return true;
        }

        let next_steps = next_steps.unwrap();

        for next_step in next_steps {
            if !self.validate_internal(visited, stack, &next_step.to.as_str()) {
                return false;
            }
        }

        stack.insert(step_id.to_string(), false);

        true
    }
}
