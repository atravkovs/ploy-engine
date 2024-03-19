use actix::{Actor, Arbiter};
use actix_rt::System;

use log::info;
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
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let arbiter_handle = Arbiter::current();
    let job_worker_actor = actors::job_worker_actor::JobWorkerActor::default().start();
    let engine_actor =
        actors::engine_actor::EngineActor::new(arbiter_handle.clone(), job_worker_actor.clone())
            .start();

    let addr = "0.0.0.0:50051".parse()?;
    let t1 = Server::builder()
        .add_service(JobWorkerServiceServer::new(MyJobWorkerService::new(
            job_worker_actor,
        )))
        .add_service(EngineServiceServer::new(MyEngineService::new(engine_actor)))
        .serve(addr);

    let t2 = actix_rt::signal::ctrl_c();

    info!("JobWorkerServer listening on {}", addr);

    select! {
        _ = t1 => {
            info!("Server stopped");
        }
        _ = t2 => {
            info!("Ctrl-C received");
        }
    };

    System::current().stop();

    Ok(())
}
