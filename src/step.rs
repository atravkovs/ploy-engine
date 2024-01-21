use std::rc::Rc;

trait Delegate {
    fn execute();
}

pub trait Step {
    fn next(&self) -> Vec<Rc<dyn Step>>;
    fn execute(&self);
}

#[derive(Clone)]
pub struct StartNode {
    next: Vec<Rc<dyn Step>>,
}

impl StartNode {
    pub fn new(next: Rc<dyn Step>) -> Self {
        Self { next: vec![next] }
    }
}

impl Step for StartNode {
    fn next(&self) -> Vec<Rc<dyn Step>> {
        self.next.clone()
    }

    fn execute(&self) {}
}

#[derive(Clone)]
pub struct EndNode {}

impl EndNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Step for EndNode {
    fn next(&self) -> Vec<Rc<dyn Step>> {
        Vec::new()
    }

    fn execute(&self) {}
}

pub struct ActivityNode {
    name: String,
    next: Vec<Rc<dyn Step>>,
    cb: Box<dyn Fn()>,
}

impl ActivityNode {
    pub fn new(name: String, cb: Box<dyn Fn()>, next: Rc<dyn Step>) -> Self {
        Self {
            name,
            cb,
            next: vec![next],
        }
    }
}

impl Step for ActivityNode {
    fn next(&self) -> Vec<Rc<dyn Step>> {
        self.next.clone()
    }

    fn execute(&self) {
        println!("I'm executing {}", self.name);

        (self.cb)();
    }
}


// fn main() {
//     let end = EndNode::new();
//     let activity2 = ActivityNode::new(
//         "Activity Two".to_string(),
//         Box::new(execute_activity),
//         Rc::new(end),
//     );

//     let activity1 = ActivityNode::new(
//         "Activity One".to_string(),
//         Box::new(execute_activity),
//         Rc::new(activity2),
//     );

//     let start = StartNode::new(Rc::new(activity1));

//     let start_c: Rc<dyn Step> = Rc::new(start);

//     execute_step(&start_c);
// }
