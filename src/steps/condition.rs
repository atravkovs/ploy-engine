use std::collections::HashMap;

use crate::definition::step::{Step, StepInputRequest};

#[derive(Debug, Clone)]
pub struct ConditionStep {
    id: String,
    inputs: Vec<StepInputRequest>,
    flows: HashMap<String, String>,
}

impl ConditionStep {
    pub fn new(id: String, inputs: Vec<StepInputRequest>, flows: HashMap<String, String>) -> Self {
        Self { id, inputs, flows }
    }
}

impl Step for ConditionStep {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn get_input_requests(&self) -> Vec<StepInputRequest> {
        self.inputs.clone()
    }

    fn get_next_steps(
        &self,
        ctx: &dyn crate::definition::step::ManageStep,
        _next_steps: &Vec<String>,
    ) -> Vec<String> {
        let mut next_steps = vec![];

        for (condition, next_step) in &self.flows {
            if let Some(condition_result) = ctx.get_inputs().get(condition) {
                if condition_result.is_boolean() && condition_result.as_bool().unwrap() {
                    next_steps.push(next_step.clone());
                }
            }
        }

        if next_steps.is_empty() && self.flows.contains_key("default") {
            next_steps.push(self.flows.get("default").unwrap().clone());
        }

        next_steps
    }
}
