use std::rc::Rc;

use anyhow::{anyhow, Result};
use quick_xml::de::from_str;
use serde::Deserialize;

use crate::step::{self, ActivityExecution, Execution, Step};

#[derive(Deserialize, PartialEq, Debug)]
struct ActivityNode {
    #[serde(rename = "@name")]
    name: String,
}

#[derive(Deserialize, PartialEq, Debug)]
enum NodeType {
    StartNode,
    EndNode,
    ActivityNode(ActivityNode),
}

#[derive(Deserialize, Debug, PartialEq)]
struct Nodes {
    #[serde(rename = "$value")]
    field: Vec<NodeType>,
}

fn execute_activity(mut execution: ActivityExecution) {
    let counter: i32 = execution
        .get_variable("counter")
        .unwrap_or(&"1".to_string())
        .parse()
        .unwrap();

    execution.set_variable("counter".to_string(), (counter + 1).to_string());

    println!(
        "Executing {} activity with name {}",
        counter,
        execution.get_name()
    );
}

fn map_node(node: &NodeType, prev: &Option<Rc<dyn Step>>) -> Rc<dyn Step> {
    match node {
        NodeType::StartNode => Rc::new(step::StartNode::new(prev.clone().unwrap())),
        NodeType::EndNode => Rc::new(step::EndNode::new()),
        NodeType::ActivityNode(activity) => Rc::new(step::ActivityNode::new(
            activity.name.to_string(),
            Box::new(execute_activity),
            prev.clone().unwrap(),
        )),
    }
}

/// Returns start node
pub fn parse_xml(xml: &str) -> Result<Rc<dyn Step>> {
    let nodes: Nodes = from_str(xml)?;

    let mut previous: Option<Rc<dyn Step>> = Option::None;

    for node in nodes.field.iter().rev() {
        previous = Some(map_node(node, &previous));
    }

    if previous.is_none() {
        return Err(anyhow!("No nodes found"));
    }

    Ok(previous.unwrap())
}
