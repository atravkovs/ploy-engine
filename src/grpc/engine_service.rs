use std::collections::HashMap;

use actix::Addr;
use serde_json::Map;
use serde_json::Value;

use crate::actors::engine_actor::{
    EngineActor, GetProcessMessage, StartProcessMessage, ValidateProcessMessage,
};

pub mod engine {
    tonic::include_proto!("org.xapik.ploy.engine");
}

pub struct MyEngineService {
    engine: Addr<EngineActor>,
}

impl MyEngineService {
    pub fn new(engine: Addr<EngineActor>) -> Self {
        Self { engine }
    }

    fn get_outputs(outputs: &Option<Map<String, Value>>) -> Value {
        match outputs {
            Some(outputs) => Value::Object(outputs.clone()),
            None => Value::Null,
        }
    }
}

#[tonic::async_trait]
impl engine::engine_service_server::EngineService for MyEngineService {
    async fn start_process(
        &self,
        request: tonic::Request<engine::StartProcessRequest>,
    ) -> Result<tonic::Response<engine::StartProcessResponse>, tonic::Status> {
        let request = request.into_inner();

        let process_name = request.process_name;

        let data: HashMap<String, Result<Value, _>> = request
            .inputs
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    serde_json::from_str(&v).map_err(|e| {
                        tonic::Status::invalid_argument(format!("Invalid input JSON: {}", e))
                    }),
                )
            })
            .collect();

        if let Some(err) = data.values().find(|v| v.is_err()) {
            return Err(err.as_ref().err().unwrap().clone());
        }

        let inputs: Map<String, Value> = data.into_iter().map(|(k, v)| (k, v.unwrap())).collect();

        let process_id = self
            .engine
            .send(StartProcessMessage {
                inputs,
                job_id: None,
                root_process_id: None,
                process_name,
            })
            .await
            .map_err(|e| tonic::Status::internal(format!("Failed to start process: {}", e)))?
            .map_err(|e| tonic::Status::internal(format!("Failed to start process: {}", e)))?;

        Ok(tonic::Response::new(engine::StartProcessResponse {
            process_id,
        }))
    }

    async fn get_process(
        &self,
        request: tonic::Request<engine::GetProcessRequest>,
    ) -> Result<tonic::Response<engine::GetProcessResponse>, tonic::Status> {
        let process_context = self
            .engine
            .send(GetProcessMessage {
                process_id: request.into_inner().process_id,
            })
            .await
            .map_err(|e| tonic::Status::internal(format!("Failed to get process: {}", e)))?
            .map_err(|e| tonic::Status::internal(format!("Failed to get process: {}", e)))?;

        Ok(tonic::Response::new(engine::GetProcessResponse {
            id: process_context.process_id.clone(),
            status: process_context.state.to_string(),
            outputs: Self::get_outputs(&process_context.outputs).to_string(),
        }))
    }

    async fn validate_process(
        &self,
        request: tonic::Request<engine::ValidateProcessRequest>,
    ) -> Result<tonic::Response<engine::ValidateProcessResponse>, tonic::Status> {
        let valid = self
            .engine
            .send(ValidateProcessMessage {
                process_name: request.into_inner().process_name,
            })
            .await
            .map_err(|e| tonic::Status::internal(format!("Failed to validate process: {}", e)))?
            .map_err(|e| tonic::Status::internal(format!("Failed to validate process: {}", e)))?;

        Ok(tonic::Response::new(engine::ValidateProcessResponse {
            valid,
        }))
    }
}
