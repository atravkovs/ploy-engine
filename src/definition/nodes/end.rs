use serde::Deserialize;

use crate::steps::end::EndStep;

use super::input_requests::InputRequests;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct EndNode {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "Inputs")]
    pub inputs: InputRequests,
}

impl Into<EndStep> for EndNode {
    fn into(self) -> EndStep {
        EndStep::new(self.id, self.inputs.into())
    }
}
