use serde::Deserialize;

use crate::definition::step::StepInputRequest;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct InputRequest {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@from")]
    pub from: String,
    #[serde(rename = "@output")]
    pub output: String,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct InputRequests {
    #[serde(rename = "$value")]
    pub inputs: Vec<InputRequest>,
}

impl Into<StepInputRequest> for InputRequest {
    fn into(self) -> StepInputRequest {
        StepInputRequest::new(self.name, self.from, self.output)
    }
}

impl Into<Vec<StepInputRequest>> for InputRequests {
    fn into(self) -> Vec<StepInputRequest> {
        self.inputs.into_iter().map(|i| i.into()).collect()
    }
}
