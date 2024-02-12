pub mod jobworker {
    tonic::include_proto!("jobworker");
}

use tonic::{transport::Server, Response};

use crate::jobworker::{
    job_worker_service_server::{JobWorkerService, JobWorkerServiceServer},
    WorkItem, WorkRequest, WorkResponse,
};

#[derive(Debug, Default)]
pub struct MyJobWorkerService {}

#[tonic::async_trait]
impl JobWorkerService for MyJobWorkerService {
    async fn get_work_items(
        &self,
        request: tonic::Request<WorkRequest>,
    ) -> Result<tonic::Response<WorkResponse>, tonic::Status> {
        println!("Got a request: {:?}", request);
        let response = WorkResponse {
            workitems: vec![WorkItem {
                inputs: "Hello World!".to_string(),
            }],
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let job_worker = MyJobWorkerService::default();

    println!("JobWorkerServer listening on {}", addr);

    Server::builder()
        .add_service(JobWorkerServiceServer::new(job_worker))
        .serve(addr)
        .await?;

    Ok(())
}
