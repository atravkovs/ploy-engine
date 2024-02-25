use actix::Actor;
use actix::Addr;
use actix::ArbiterHandle;

use crate::actors::job_worker_actor::JobWorkerActor;
use crate::actors::process_actor::ProcessActor;

pub mod engine {
    tonic::include_proto!("org.xapik.ploy.engine");
}

#[derive(Debug)]
pub struct MyEngineService {
    arbiter_handle: actix::ArbiterHandle,
    job_worker_actor: Addr<JobWorkerActor>,
}

impl MyEngineService {
    pub fn new(job_worker_actor: Addr<JobWorkerActor>, arbiter_handle: ArbiterHandle) -> Self {
        MyEngineService {
            job_worker_actor,
            arbiter_handle,
        }
    }
}

#[tonic::async_trait]
impl engine::engine_service_server::EngineService for MyEngineService {
    async fn start_process(
        &self,
        _request: tonic::Request<engine::StartProcessRequest>,
    ) -> Result<tonic::Response<engine::StartProcessResponse>, tonic::Status> {
        let process_actor = self.job_worker_actor.clone();
        ProcessActor::start_in_arbiter(&self.arbiter_handle, |_ctx| {
            crate::actors::process_actor::ProcessActor::new(process_actor)
        });

        Ok(tonic::Response::new(engine::StartProcessResponse {}))
    }
}
