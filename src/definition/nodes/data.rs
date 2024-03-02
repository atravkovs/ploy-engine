use serde::Deserialize;

use crate::steps::data::DataStep;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct DataNodeOutput {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct DataNodeOutputs {
    #[serde(rename = "$value")]
    pub outputs: Vec<DataNodeOutput>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct DataNode {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "Outputs")]
    pub outputs: DataNodeOutputs,
}

impl Into<DataStep> for DataNode {
    fn into(self) -> DataStep {
        DataStep::new(self.id)
    }
}
