use std::collections::HashMap;

use anyhow::{anyhow, Result};
use quick_xml::de::from_str;
use serde::Deserialize;

use crate::engine::step;

use super::step::StepLeaf;

#[derive(Deserialize, PartialEq, Debug)]
struct ActivityInput {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@value")]
    value: String,
}

#[derive(Deserialize, PartialEq, Debug)]
enum ActivityValueType {
    Input(ActivityInput),
}

#[derive(Deserialize, PartialEq, Debug)]
struct SimpleNode {
    #[serde(rename = "@id")]
    id: String,
}

#[derive(Deserialize, PartialEq, Debug)]
struct ActivityNode {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@job")]
    job: String,
    #[serde(rename = "$value")]
    field: Vec<ActivityValueType>,
}

#[derive(Deserialize, PartialEq, Debug)]
enum NodeType {
    StartNode(SimpleNode),
    EndNode(SimpleNode),
    ActivityNode(ActivityNode),
}

#[derive(Deserialize, Debug, PartialEq)]
struct Nodes {
    #[serde(rename = "$value")]
    field: Vec<NodeType>,
}

fn get_activity_inputs(activity: &ActivityNode) -> HashMap<String, String> {
    activity
        .field
        .iter()
        .flat_map(|field| match field {
            ActivityValueType::Input(input) => vec![(input.name.clone(), input.value.clone())],
        })
        .collect()
}

fn map_node(node: &NodeType, prev: Option<StepLeaf>) -> StepLeaf {
    let mut leaf = match node {
        NodeType::StartNode(simple) => {
            StepLeaf::new(simple.id.clone(), step::ActivityType::StartNode)
        }
        NodeType::EndNode(simple) => StepLeaf::new(simple.id.clone(), step::ActivityType::EndNode),
        NodeType::ActivityNode(activity) => StepLeaf::new(
            activity.id.clone(),
            step::ActivityType::ActivityNode(step::ActivityNode::new(
                activity.name.to_string(),
                activity.job.to_string(),
                get_activity_inputs(activity),
            )),
        ),
    };

    if let Some(prev) = prev {
        leaf.add_next(prev);
    }

    leaf
}

/// Returns start node
pub fn parse_xml(xml: &str) -> Result<StepLeaf> {
    let nodes: Nodes = from_str(xml)?;

    let mut previous: Option<StepLeaf> = Option::None;

    for node in nodes.field.iter().rev() {
        previous = Some(map_node(node, previous));
    }

    if previous.is_none() {
        return Err(anyhow!("No nodes found"));
    }

    Ok(previous.unwrap())
}
