use std::rc::Rc;

use actix::{Actor, Addr, Handler, Message};

use crate::engine::step::Step;

use super::job_worker_actor::JobWorkerActor;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ExecuteProcess;

pub struct ProcessActor {
    job_worker: Addr<JobWorkerActor>,
    start_node: Option<Rc<dyn Step>>,
}

impl ProcessActor {
    pub fn new(job_worker: Addr<JobWorkerActor>) -> Self {
        Self {
            job_worker,
            start_node: Option::None,
        }
    }
}

impl Actor for ProcessActor {
    type Context = actix::Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        let file_name = "data/Test.ploy";
        let file_contents = std::fs::read_to_string(file_name).unwrap();

        let start_step = crate::engine::parser::parse_xml(&file_contents).unwrap();

        self.start_node = Some(start_step);
    }
}

impl Handler<ExecuteProcess> for ProcessActor {
    type Result = ();

    fn handle(&mut self, _msg: ExecuteProcess, _ctx: &mut Self::Context) -> Self::Result {
        let mut ctx = crate::engine::ProcessContext::new(self.job_worker.clone());
        crate::engine::execute_step(self.start_node.as_ref().unwrap(), &mut ctx);
    }
}
