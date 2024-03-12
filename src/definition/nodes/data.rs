use serde::Deserialize;

use crate::steps::data::DataStep;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum DataNodeTypes {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct DataNode {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub rtype: DataNodeTypes,
    #[serde(rename = "@value")]
    pub value: String,
}

impl Into<DataStep> for DataNode {
    fn into(self) -> DataStep {
        let value = match self.rtype {
            DataNodeTypes::String => serde_json::Value::String(self.value),
            DataNodeTypes::Number => serde_json::Value::Number(self.value.parse().unwrap()),
            DataNodeTypes::Boolean => serde_json::Value::Bool(self.value.parse().unwrap()),
        };

        DataStep::new(self.id, value)
    }
}
