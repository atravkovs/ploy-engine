use std::rc::Rc;

use crate::ProcessContext;

pub trait Execution {
    fn get_name(&self) -> &str;
}

#[derive(Debug)]
pub struct ActivityExecution<'a> {
    name: String,
    ctx: &'a mut ProcessContext,
}

impl<'a> ActivityExecution<'a> {
    pub fn new(name: String, ctx: &'a mut ProcessContext) -> Self {
        Self { name, ctx }
    }

    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.ctx.get_variable(name)
    }

    pub fn set_variable(&mut self, name: String, value: String) {
        self.ctx.set_variable(name, value);
    }
}

impl<'a> Execution for ActivityExecution<'a> {
    fn get_name(&self) -> &str {
        &self.name
    }
}

pub trait Step {
    fn next(&self) -> Vec<Rc<dyn Step>>;
    fn execute(&self, ctx: &mut ProcessContext);
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

    fn execute(&self, _ctx: &mut ProcessContext) {}
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

    fn execute(&self, _ctx: &mut ProcessContext) {}
}

pub struct ActivityNode {
    name: String,
    next: Vec<Rc<dyn Step>>,
    cb: Box<dyn Fn(ActivityExecution)>,
}

impl ActivityNode {
    pub fn new(name: String, cb: Box<dyn Fn(ActivityExecution)>, next: Rc<dyn Step>) -> Self {
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

    fn execute(&self, ctx: &mut ProcessContext) {
        let execution = ActivityExecution::new(self.name.clone(), ctx);

        (self.cb)(execution);
    }
}
