use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct FlowNode {
    #[serde(rename = "@from")]
    from: String,
    #[serde(rename = "@to")]
    to: String,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct FlowNodes {
    #[serde(rename = "FlowNode")]
    nodes: Vec<FlowNode>,
}

impl Into<HashMap<String, Vec<String>>> for FlowNodes {
    fn into(self) -> HashMap<String, Vec<String>> {
        let mut flow: HashMap<String, Vec<String>> = HashMap::default();

        for n in self.nodes.iter() {
            let from = n.from.clone();
            let to = n.to.clone();

            if let Some(f) = flow.get_mut(&from) {
                f.push(to);
            } else {
                flow.insert(from, vec![to]);
            }
        }

        flow
    }
}
