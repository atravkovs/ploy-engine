pub mod jobworker {
    tonic::include_proto!("org.xapik.ploy.jobworker");
}

use actix::Addr;
use serde_json::{Map, Value};
use tonic::Response;

use crate::actors::job_worker_actor::{GetWorkItems, JobCompletedMessage, JobWorkerActor};

use self::jobworker::{
    job_worker_service_server::JobWorkerService, CompleteWorkItemRequest, CompleteWorkItemResponse,
    WorkItem, WorkRequest, WorkResponse,
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
    async fn complete_work_item(
        &self,
        request: tonic::Request<CompleteWorkItemRequest>,
    ) -> Result<tonic::Response<CompleteWorkItemResponse>, tonic::Status> {
        let inner_request = request.into_inner();

        let job_outputs: Map<String, Value> = serde_json::from_str(&inner_request.outputs)
            .map_err(|e| {
                log::error!("Error: {:?}", e);
                tonic::Status::invalid_argument("Invalid outputs JSON")
            })?;

        let message = JobCompletedMessage::new(inner_request.job_id, job_outputs);
        self.job_worker_actor.do_send(message);

        Ok(Response::new(CompleteWorkItemResponse {}))
    }

    async fn get_work_items(
        &self,
        _request: tonic::Request<WorkRequest>,
    ) -> Result<tonic::Response<WorkResponse>, tonic::Status> {
        let job_items = self
            .job_worker_actor
            .send(GetWorkItems)
            .await
            .map_err(|e| {
                log::error!("Error: {:?}", e);
                tonic::Status::internal("Internal error")
            })?;

        let response = WorkResponse {
            workitems: job_items
                .into_iter()
                .map(|i| WorkItem {
                    job_id: i.id,
                    inputs: i.inputs,
                    job_name: i.job_name,
                })
                .collect(),
        };

        Ok(Response::new(response))
    }
}
