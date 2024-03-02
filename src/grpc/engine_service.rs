use std::sync::Arc;

use actix::Actor;
use actix::Addr;
use actix::ArbiterHandle;
use serde_json::{Map, Value};

use crate::actors::job_worker_actor::JobWorkerActor;
use crate::actors::process_actor::ProcessActor;
use crate::definition::process_definition::ProcessDefinition;

pub mod engine {
    tonic::include_proto!("org.xapik.ploy.engine");
}

pub struct MyEngineService {
    arbiter_handle: actix::ArbiterHandle,
    job_worker_actor: Addr<JobWorkerActor>,
    process_definition: Arc<ProcessDefinition>,
}

impl MyEngineService {
    pub fn new(
        job_worker_actor: Addr<JobWorkerActor>,
        arbiter_handle: ArbiterHandle,
        process_definition: Arc<ProcessDefinition>,
    ) -> Self {
        MyEngineService {
            job_worker_actor,
            arbiter_handle,
            process_definition,
        }
    }
}

#[tonic::async_trait]
impl engine::engine_service_server::EngineService for MyEngineService {
    async fn start_process(
        &self,
        request: tonic::Request<engine::StartProcessRequest>,
    ) -> Result<tonic::Response<engine::StartProcessResponse>, tonic::Status> {
        let process_actor = self.job_worker_actor.clone();
        let process = self.process_definition.clone();

        let inputs: Map<String, Value> = request
            .into_inner()
            .inputs
            .into_iter()
            .map(|(k, v)| (k, Value::String(v.into())))
            .collect();

        ProcessActor::start_in_arbiter(&self.arbiter_handle, |_ctx| {
            crate::actors::process_actor::ProcessActor::new(process_actor, process, inputs)
        });

        Ok(tonic::Response::new(engine::StartProcessResponse {}))
    }
}
