use std::{collections::HashMap, rc::Rc};

use actix::Addr;

use crate::actors::job_worker_actor::JobWorkerActor;

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

    pub fn add_job(&self, inputs: String) {
        self.job_worker
            .do_send(crate::actors::job_worker_actor::AddWorkItem(inputs));
    }
}

pub fn execute_step(step: &Rc<dyn step::Step>, ctx: &mut ProcessContext) {
    step.execute(ctx);

    let next_steps = step.next();

    for next_step in next_steps.iter() {
        execute_step(next_step, ctx);
    }
}
