use serde::Deserialize;

use crate::steps::script::ScriptStep;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct ScriptNode {
    #[serde(rename = "@id")]
    pub id: String,
}

impl Into<ScriptStep> for ScriptNode {
    fn into(self) -> ScriptStep {
        ScriptStep::new(self.id)
    }
}
