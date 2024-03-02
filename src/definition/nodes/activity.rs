use serde::Deserialize;

use crate::{definition::step::StepInputRequest, steps::activity::ActivityStep};

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct ActivityInput {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@from")]
    pub from: String,
    #[serde(rename = "@output")]
    pub output: String,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct ActivityInputs {
    #[serde(rename = "$value")]
    pub inputs: Vec<ActivityInput>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct ActivityNode {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@job")]
    pub job: String,
    #[serde(rename = "Inputs")]
    pub inputs: ActivityInputs,
}

impl Into<StepInputRequest> for ActivityInput {
    fn into(self) -> StepInputRequest {
        StepInputRequest::new(self.name, self.from, self.output)
    }
}

impl Into<Vec<StepInputRequest>> for ActivityInputs {
    fn into(self) -> Vec<StepInputRequest> {
        self.inputs.into_iter().map(|i| i.into()).collect()
    }
}

impl Into<ActivityStep> for ActivityNode {
    fn into(self) -> ActivityStep {
        ActivityStep::new(self.id, self.name, self.job, self.inputs.into())
    }
}
