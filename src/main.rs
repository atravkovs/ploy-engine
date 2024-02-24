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
    let file_name = "data/Test.ploy";
    let file_contents = std::fs::read_to_string(file_name).unwrap();

    let start_step = crate::engine::parser::parse_xml(&file_contents).unwrap();

    let job_worker_actor = actors::job_worker_actor::JobWorkerActor::default().start();
    actors::process_actor::ProcessActor::new(job_worker_actor.clone(), start_step).start();

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
