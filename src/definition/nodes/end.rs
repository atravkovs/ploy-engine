use serde::Deserialize;

use crate::steps::end::EndStep;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct EndNode {
    #[serde(rename = "@id")]
    pub id: String,
}

impl Into<EndStep> for EndNode {
    fn into(self) -> EndStep {
        EndStep::new(self.id)
    }
}
