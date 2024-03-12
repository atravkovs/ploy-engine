use std::sync::Arc;

use actix::{Actor, Arbiter};
use actix_rt::System;

use tokio::select;
use tonic::transport::Server;

use crate::grpc::{
    engine_service::{engine::engine_service_server::EngineServiceServer, MyEngineService},
    job_worker_service::{
        jobworker::job_worker_service_server::JobWorkerServiceServer, MyJobWorkerService,
    },
};

pub mod actors;
pub mod definition;
pub mod grpc;
pub mod steps;

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = "data/Test-v2.ploy";
    let file_contents = std::fs::read_to_string(file_name).unwrap();

    let process_definition = Arc::new(
        crate::definition::parser::parse_xml(&file_contents)
            .expect("Error parsing process definition"),
    );

    let job_worker_actor = actors::job_worker_actor::JobWorkerActor::default().start();

    let addr = "0.0.0.0:50051".parse()?;
    let job_worker_service = MyJobWorkerService::new(job_worker_actor.clone());

    println!("JobWorkerServer listening on {}", addr);

    let arbiter_handle = Arbiter::current();

    let t1 = Server::builder()
        .add_service(JobWorkerServiceServer::new(job_worker_service))
        .add_service(EngineServiceServer::new(MyEngineService::new(
            job_worker_actor.clone(),
            arbiter_handle,
            process_definition,
        )))
        .serve(addr);

    let t2 = actix_rt::signal::ctrl_c();

    select! {
        _ = t1 => {
            println!("\nServer stopped");
        }
        _ = t2 => {
            println!("\nCtrl-C received");
        }
    };

    System::current().stop();

    Ok(())
}
