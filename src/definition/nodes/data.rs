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
        let mut outputs = serde_json::Map::new();

        self.outputs.outputs.iter().for_each(|o| {
            outputs.insert(o.name.clone(), serde_json::Value::String(o.value.clone()));
        });

        DataStep::new(self.id, outputs)
    }
}
