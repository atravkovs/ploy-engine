use crate::definition::step::{FlowLeaf, Step, StepInputRequest};

#[derive(Debug, Clone)]
pub struct ConditionStep {
    id: String,
    inputs: Vec<StepInputRequest>,
}

impl ConditionStep {
    pub fn new(id: String, inputs: Vec<StepInputRequest>) -> Self {
        Self { id, inputs }
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
        next_steps: &Vec<FlowLeaf>,
    ) -> Vec<String> {
        let inputs = ctx.get_inputs();

        let steps: Vec<String> = next_steps
            .iter()
            .filter(|n| {
                if let Some(input) = &n.input {
                    inputs.contains_key(input)
                        && inputs.get(input).unwrap().is_boolean()
                        && inputs.get(input).unwrap().as_bool().unwrap()
                } else {
                    false
                }
            })
            .map(|n| n.to.clone())
            .collect();

        if steps.is_empty() {
            next_steps
                .iter()
                .filter(|n| n.input.is_none())
                .map(|n| n.to.clone())
                .collect()
        } else {
            steps
        }
    }
}
