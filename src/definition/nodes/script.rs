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
    #[serde(rename = "@input")]
    pub input: String,
    #[serde(rename = "@output")]
    pub output: String,
}

impl Into<ScriptStep> for ScriptNode {
    fn into(self) -> ScriptStep {
        ScriptStep::new(
            self.id,
            self.script,
            self.input,
            self.output,
            self.inputs.into(),
        )
    }
}
