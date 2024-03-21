use serde::Deserialize;

use crate::steps::condition::ConditionStep;

use super::input_requests::InputRequests;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct ConditionNode {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "Inputs")]
    pub inputs: InputRequests,
}

impl Into<ConditionStep> for ConditionNode {
    fn into(self) -> ConditionStep {
        ConditionStep::new(self.id, self.inputs.into())
    }
}
