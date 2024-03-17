use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    definition::step::Step,
    steps::{
        activity::ActivityStep, call::CallStep, condition::ConditionStep, data::DataStep,
        end::EndStep, script::ScriptStep, start::StartStep,
    },
};

use super::{
    activity::ActivityNode, call::CallNode, condition::ConditionNode, data::DataNode, end::EndNode,
    script::ScriptNode, start::StartNode,
};

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum NodeType {
    StartNode(StartNode),
    EndNode(EndNode),
    ActivityNode(ActivityNode),
    DataNode(DataNode),
    ScriptNode(ScriptNode),
    ConditionNode(ConditionNode),
    CallNode(CallNode),
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Nodes {
    #[serde(rename = "$value")]
    pub field: Vec<NodeType>,
}

impl Into<HashMap<String, Box<dyn Step>>> for Nodes {
    fn into(self) -> HashMap<String, Box<dyn Step>> {
        let mut steps: HashMap<String, Box<dyn Step>> = HashMap::default();

        for n in self.field.iter() {
            match n {
                NodeType::StartNode(start) => {
                    let start_step: StartStep = start.clone().into();

                    steps.insert(start.id.clone(), Box::new(start_step));
                }
                NodeType::EndNode(end) => {
                    let end_step: EndStep = end.clone().into();

                    steps.insert(end.id.clone(), Box::new(end_step));
                }
                NodeType::ActivityNode(activity) => {
                    let activity_step: ActivityStep = activity.clone().into();

                    steps.insert(activity.id.clone(), Box::new(activity_step));
                }
                NodeType::DataNode(data) => {
                    let data_step: DataStep = data.clone().into();

                    steps.insert(data.id.clone(), Box::new(data_step));
                }
                NodeType::ScriptNode(script) => {
                    let script_step: ScriptStep = script.clone().into();

                    steps.insert(script.id.clone(), Box::new(script_step));
                }
                NodeType::ConditionNode(condition) => {
                    let condition_step: ConditionStep = condition.clone().into();

                    steps.insert(condition.id.clone(), Box::new(condition_step));
                }
                NodeType::CallNode(call) => {
                    let call_step: CallStep = call.clone().into();

                    steps.insert(call.id.clone(), Box::new(call_step));
                }
            }
        }

        steps
    }
}
