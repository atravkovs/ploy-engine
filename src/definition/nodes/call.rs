use serde::Deserialize;

use crate::steps::call::CallStep;

use super::input_requests::InputRequests;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct CallNode {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@process")]
    pub process: String,
    #[serde(rename = "Inputs")]
    pub inputs: InputRequests,
}

impl Into<CallStep> for CallNode {
    fn into(self) -> CallStep {
        CallStep::new(self.id, self.process, self.inputs.into())
    }
}
