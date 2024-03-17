use actix::Addr;
use serde_json::Map;
use serde_json::Value;

use crate::actors::engine_actor::EngineActor;
use crate::actors::engine_actor::GetProcessMessage;
use crate::actors::engine_actor::StartProcessMessage;

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
        let inputs = request
            .inputs
            .into_iter()
            .map(|(k, v)| (k, Value::String(v.into())))
            .collect();

        let process_id = self
            .engine
            .send(StartProcessMessage {
                process_name,
                inputs,
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
}
