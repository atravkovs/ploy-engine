use std::collections::HashMap;

use actix::Addr;

use crate::actors::job_worker_actor::{JobItem, JobWorkerActor};

pub mod parser;
pub mod step;

#[derive(Debug, Clone)]
pub struct ProcessContext {
    state: HashMap<String, String>,
    job_worker: Addr<JobWorkerActor>,
}

impl ProcessContext {
    pub fn new(job_worker: Addr<JobWorkerActor>) -> Self {
        ProcessContext {
            state: HashMap::new(),
            job_worker,
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.state.get(name)
    }

    pub fn set_variable(&mut self, name: String, value: String) {
        self.state.insert(name, value);
    }

    pub fn add_job(&self, inputs: JobItem) {
        self.job_worker
            .do_send(crate::actors::job_worker_actor::AddWorkItem(inputs));
    }
}

pub trait Step {
    fn id(&self) -> String;
    fn activity_id(&self) -> String;
    fn start(&mut self, ctx: &mut ProcessContext) {
        self.end(ctx);
    }
    fn end(&mut self, _ctx: &mut ProcessContext) {}
    fn status(&self) -> StepStatus {
        StepStatus::Completed
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepStatus {
    Open,
    InProgress,
    Completed,
}
