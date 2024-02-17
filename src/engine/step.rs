use std::{collections::HashMap, rc::Rc};

use crate::{actors::job_worker_actor::JobItem, engine::ProcessContext};

pub trait Execution {
    fn get_name(&self) -> &str;
    fn get_job(&self) -> &str;
}

#[derive(Debug)]
pub struct ActivityExecution<'a> {
    name: &'a str,
    job: &'a str,
    inputs: &'a HashMap<String, String>,
    ctx: &'a mut ProcessContext,
}

impl<'a> ActivityExecution<'a> {
    pub fn new(
        name: &'a str,
        job: &'a str,
        inputs: &'a HashMap<String, String>,
        ctx: &'a mut ProcessContext,
    ) -> Self {
        Self {
            name,
            job,
            inputs,
            ctx,
        }
    }

    pub fn get_input(&self, name: &str) -> Option<&String> {
        self.inputs.get(name)
    }

    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.ctx.get_variable(name)
    }

    pub fn set_variable(&mut self, name: String, value: String) {
        self.ctx.set_variable(name, value);
    }

    pub fn add_job(&self, job: JobItem) {
        self.ctx.add_job(job);
    }
}

impl<'a> Execution for ActivityExecution<'a> {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_job(&self) -> &str {
        &self.job
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
    job: String,
    next: Vec<Rc<dyn Step>>,
    cb: Box<dyn Fn(ActivityExecution)>,
    inputs: HashMap<String, String>,
}

impl ActivityNode {
    pub fn new(
        name: String,
        job: String,
        cb: Box<dyn Fn(ActivityExecution)>,
        inputs: HashMap<String, String>,
        next: Rc<dyn Step>,
    ) -> Self {
        Self {
            name,
            job,
            cb,
            inputs,
            next: vec![next],
        }
    }
}

impl Step for ActivityNode {
    fn next(&self) -> Vec<Rc<dyn Step>> {
        self.next.clone()
    }

    fn execute(&self, ctx: &mut ProcessContext) {
        let execution = ActivityExecution::new(&self.name, &self.job, &self.inputs, ctx);

        (self.cb)(execution);
    }
}
