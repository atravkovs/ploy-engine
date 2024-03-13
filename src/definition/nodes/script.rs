use serde::Deserialize;

use crate::steps::script::ScriptStep;

use super::input_requests::InputRequests;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct ScriptNode {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@script")]
    pub script: String,
    #[serde(rename = "Inputs")]
    pub inputs: InputRequests,
}

impl Into<ScriptStep> for ScriptNode {
    fn into(self) -> ScriptStep {
        ScriptStep::new(self.id, self.script, self.inputs.into())
    }
}
