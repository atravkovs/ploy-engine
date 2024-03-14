use std::collections::HashMap;

use serde::Deserialize;

use crate::steps::condition::ConditionStep;

use super::input_requests::InputRequests;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct ConditionFlow {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@to")]
    pub to: String,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct ConditionFlows {
    #[serde(rename = "$value")]
    pub inputs: Vec<ConditionFlow>,
}

impl Into<HashMap<String, String>> for ConditionFlows {
    fn into(self) -> HashMap<String, String> {
        let mut flows = HashMap::default();

        for flow in self.inputs {
            flows.insert(flow.name, flow.to);
        }

        flows
    }
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct ConditionNode {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "Inputs")]
    pub inputs: InputRequests,
    #[serde(rename = "Flows")]
    pub flows: ConditionFlows,
}

impl Into<ConditionStep> for ConditionNode {
    fn into(self) -> ConditionStep {
        ConditionStep::new(self.id, self.inputs.into(), self.flows.into())
    }
}
