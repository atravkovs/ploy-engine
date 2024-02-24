use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct StepLeaf {
    pub id: String,
    pub step: ActivityType,
    pub next: Vec<Rc<RefCell<StepLeaf>>>,
}

impl StepLeaf {
    pub fn new(id: String, step: ActivityType) -> Self {
        Self {
            id,
            step,
            next: Vec::new(),
        }
    }

    pub fn add_next(&mut self, next: StepLeaf) {
        self.next.push(Rc::new(RefCell::new(next)));
    }

    pub fn next_for(&self, id: &str) -> Vec<Rc<RefCell<StepLeaf>>> {
        if id == self.id {
            self.next.clone()
        } else {
            self.next
                .iter()
                .flat_map(|next| next.borrow().next_for(id))
                .collect()
        }
    }
}

impl StepLeaf {
    pub fn step(&self) -> &ActivityType {
        &self.step
    }

    pub fn next(&self) -> &Vec<Rc<RefCell<StepLeaf>>> {
        &self.next
    }
}

#[derive(Debug, Clone)]
pub struct ActivityNode {
    pub name: String,
    pub job: String,
    pub inputs: HashMap<String, String>,
}

impl ActivityNode {
    pub fn new(name: String, job: String, inputs: HashMap<String, String>) -> Self {
        Self { name, job, inputs }
    }
}

#[derive(Debug, Clone)]
pub enum ActivityType {
    StartNode,
    EndNode,
    ActivityNode(ActivityNode),
}
