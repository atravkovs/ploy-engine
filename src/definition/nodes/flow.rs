use std::collections::HashMap;

use serde::Deserialize;

use crate::definition::step::FlowLeaf;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct FlowNode {
    #[serde(rename = "@from")]
    from: String,
    #[serde(rename = "@to")]
    to: String,
    #[serde(rename = "@input")]
    input: Option<String>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct FlowNodes {
    #[serde(rename = "FlowNode")]
    nodes: Vec<FlowNode>,
}

impl Into<HashMap<String, Vec<FlowLeaf>>> for FlowNodes {
    fn into(self) -> HashMap<String, Vec<FlowLeaf>> {
        let mut flow: HashMap<String, Vec<FlowLeaf>> = HashMap::default();

        for n in self.nodes.iter() {
            let from = n.from.clone();
            let to = n.to.clone();
            let input = n.input.clone();

            if let Some(f) = flow.get_mut(&from) {
                f.push(FlowLeaf { to, input });
            } else {
                flow.insert(from, vec![FlowLeaf { to, input }]);
            }
        }

        flow
    }
}
