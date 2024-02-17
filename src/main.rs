use actix::Actor;
use actix_rt::System;
use tonic::transport::Server;

use crate::grpc::{
    jobworker::job_worker_service_server::JobWorkerServiceServer, MyJobWorkerService,
};

pub mod actors;
pub mod engine;
pub mod grpc;

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let job_worker_actor = actors::job_worker_actor::JobWorkerActor::default().start();
    let process_actor = actors::process_actor::ProcessActor::new(job_worker_actor.clone()).start();

    process_actor
        .send(actors::process_actor::ExecuteProcess)
        .await?;

    let addr = "0.0.0.0:50051".parse()?;
    let job_worker_service = MyJobWorkerService::new(job_worker_actor);

    println!("JobWorkerServer listening on {}", addr);

    Server::builder()
        .add_service(JobWorkerServiceServer::new(job_worker_service))
        .serve(addr)
        .await?;

    System::current().stop();

    Ok(())
}
