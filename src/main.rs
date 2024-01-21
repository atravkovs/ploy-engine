use anyhow::Result;
use quick_xml::de::from_str;
use serde::Deserialize;
use std::fs;

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

fn main() -> Result<()> {
    let file_name = "Test.ploy";

    let file_contents = fs::read_to_string(file_name)?;

    let nodes: Nodes = from_str(&file_contents)?;

    println!("{:?}", nodes);

    Ok(())
}
