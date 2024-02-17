pub mod jobworker {
    tonic::include_proto!("org.xapik.ploy.jobworker");
}

use actix::Addr;
use tonic::Response;

use crate::{
    actors::job_worker_actor::{GetWorkItems, JobWorkerActor},
    grpc::jobworker::{
        job_worker_service_server::JobWorkerService, WorkItem, WorkRequest, WorkResponse,
    },
};

#[derive(Debug)]
pub struct MyJobWorkerService {
    job_worker_actor: Addr<JobWorkerActor>,
}

impl MyJobWorkerService {
    pub fn new(job_worker_actor: Addr<JobWorkerActor>) -> Self {
        MyJobWorkerService { job_worker_actor }
    }
}

#[tonic::async_trait]
impl JobWorkerService for MyJobWorkerService {
    async fn get_work_items(
        &self,
        request: tonic::Request<WorkRequest>,
    ) -> Result<tonic::Response<WorkResponse>, tonic::Status> {
        println!("Got a request: {:?}", request);

        let job_items = self
            .job_worker_actor
            .send(GetWorkItems)
            .await
            .map_err(|e| {
                println!("Error: {:?}", e);
                tonic::Status::internal("Internal error")
            })?;

        let response = WorkResponse {
            workitems: job_items
                .into_iter()
                .map(|i| WorkItem {
                    inputs: i.inputs,
                    job_name: i.job_name,
                })
                .collect(),
        };

        Ok(Response::new(response))
    }
}
