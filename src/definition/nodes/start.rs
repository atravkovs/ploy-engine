use serde::Deserialize;

use crate::steps::start::StartStep;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct StartNode {
    #[serde(rename = "@id")]
    pub id: String,
}

impl Into<StartStep> for StartNode {
    fn into(self) -> StartStep {
        StartStep::new(self.id)
    }
}
