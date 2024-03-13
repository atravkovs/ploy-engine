use serde::Deserialize;

use crate::steps::activity::ActivityStep;

use super::input_requests::InputRequests;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct ActivityNode {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@input")]
    pub input: String,
    #[serde(rename = "@output")]
    pub output: String,
    #[serde(rename = "@job")]
    pub job: String,
    #[serde(rename = "Inputs")]
    pub inputs: InputRequests,
}

impl Into<ActivityStep> for ActivityNode {
    fn into(self) -> ActivityStep {
        ActivityStep::new(
            self.id,
            self.name,
            self.input,
            self.output,
            self.job,
            self.inputs.into(),
        )
    }
}
