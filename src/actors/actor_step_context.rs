use actix::Addr;
use serde_json::{Map, Value};
use uuid::Uuid;

use crate::definition::step::ManageStep;

use super::job_worker_actor::{JobItem, JobWorkerActor};

#[derive(Debug, Clone)]
pub struct ActorStepContext {
    job_worker: Addr<JobWorkerActor>,
    inputs: Map<String, Value>,
}

impl ActorStepContext {
    pub fn new(job_worker: Addr<JobWorkerActor>, inputs: Map<String, Value>) -> Self {
        ActorStepContext { job_worker, inputs }
    }

    pub fn job_worker(&self) -> Addr<JobWorkerActor> {
        self.job_worker.clone()
    }
}

impl ManageStep for ActorStepContext {
    fn add_job(&self, job_name: String) -> String {
        let id = Uuid::new_v4().to_string();

        self.job_worker
            .do_send(crate::actors::job_worker_actor::AddWorkItem(JobItem::new(
                id.clone(),
                Value::Object(self.get_inputs().clone()).to_string(),
                job_name,
            )));

        id
    }

    fn get_inputs(&self) -> &Map<String, Value> {
        &self.inputs
    }
}
