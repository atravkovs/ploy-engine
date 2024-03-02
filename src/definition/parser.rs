use anyhow::Result;
use quick_xml::de::from_str;
use serde::Deserialize;

use super::{
    nodes::{flow::FlowNodes, node::Nodes},
    process_definition::ProcessDefinition,
};

#[derive(Deserialize, PartialEq, Debug)]
struct PloyDefinitionXml {
    #[serde(rename = "Nodes")]
    nodes: Nodes,
    #[serde(rename = "Flow")]
    flow: FlowNodes,
}

fn get_start_step(nodes: &Nodes) -> Result<String> {
    let start_nodes: Vec<String> = nodes
        .field
        .iter()
        .filter_map(|n| match n {
            super::nodes::node::NodeType::StartNode(start) => Some(start.id.clone()),
            _ => None,
        })
        .collect();

    if start_nodes.len() > 1 {
        return Err(anyhow::anyhow!(
            "More than one start node found in the process definition"
        ));
    }

    start_nodes
        .first()
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("No start node found in the process definition"))
}

pub fn parse_xml(xml: &str) -> Result<ProcessDefinition> {
    let ploy: PloyDefinitionXml = from_str(xml)?;

    let steps = ploy.nodes.clone().into();
    let flow = ploy.flow.clone().into();
    let start_step_id = get_start_step(&ploy.nodes)?;

    Ok(ProcessDefinition::new(steps, flow, start_step_id))
}
