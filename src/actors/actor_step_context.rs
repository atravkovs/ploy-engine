use actix::Addr;
use serde_json::{Map, Value};
use uuid::Uuid;

use crate::definition::step::ManageStep;

use super::{
    engine_actor::{EngineActor, StartProcessMessage},
    job_worker_actor::{JobItem, JobWorkerActor},
};

#[derive(Debug, Clone)]
pub struct ActorStepContext {
    process_id: String,
    engine: Addr<EngineActor>,
    job_worker: Addr<JobWorkerActor>,
    inputs: Map<String, Value>,
}

impl ActorStepContext {
    pub fn new(
        process_id: String,
        engine: Addr<EngineActor>,
        job_worker: Addr<JobWorkerActor>,
        inputs: Map<String, Value>,
    ) -> Self {
        ActorStepContext {
            process_id,
            engine,
            job_worker,
            inputs,
        }
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

    fn start_process(
        &self,
        process_name: String,
        inputs: Map<String, Value>,
    ) -> anyhow::Result<crate::definition::step::JobId> {
        let id = Uuid::new_v4().to_string();

        self.engine.do_send(StartProcessMessage {
            root_process_id: Some(self.process_id.clone()),
            job_id: Some(id.clone()),
            process_name,
            inputs,
        });

        Ok(id)
    }
}
