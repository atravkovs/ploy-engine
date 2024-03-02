use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    definition::step::Step,
    steps::{activity::ActivityStep, data::DataStep, end::EndStep, start::StartStep},
};

use super::{activity::ActivityNode, data::DataNode, end::EndNode, start::StartNode};

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum NodeType {
    StartNode(StartNode),
    EndNode(EndNode),
    ActivityNode(ActivityNode),
    DataNode(DataNode),
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
            }
        }

        steps
    }
}
