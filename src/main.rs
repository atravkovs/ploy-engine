use actix::{Actor, Arbiter};
use actix_rt::System;
use tonic::transport::Server;

use crate::grpc::{
    engine_service::{engine::engine_service_server::EngineServiceServer, MyEngineService},
    job_worker_service::{
        jobworker::job_worker_service_server::JobWorkerServiceServer, MyJobWorkerService,
    },
};

pub mod actors;
pub mod engine;
pub mod grpc;

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let job_worker_actor = actors::job_worker_actor::JobWorkerActor::default().start();

    let addr = "0.0.0.0:50051".parse()?;
    let job_worker_service = MyJobWorkerService::new(job_worker_actor.clone());

    println!("JobWorkerServer listening on {}", addr);

    let arbiter_handle = Arbiter::current();

    Server::builder()
        .add_service(JobWorkerServiceServer::new(job_worker_service))
        .add_service(EngineServiceServer::new(MyEngineService::new(
            job_worker_actor.clone(),
            arbiter_handle,
        )))
        .serve(addr)
        .await?;

    System::current().stop();

    Ok(())
}
