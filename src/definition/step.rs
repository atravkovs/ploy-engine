use anyhow::Result;
use serde_json::{Map, Value};

pub type JobId = String;
pub type StepOutputs = Map<String, Value>;

pub trait ManageStep {
    fn add_job(&self, job_name: String) -> JobId;
    fn get_inputs(&self) -> &Map<String, Value>;
}

#[derive(Debug, Clone)]
pub struct StepInputRequest {
    pub name: String,
    pub from: String,
    pub output: String,
}

impl StepInputRequest {
    pub fn new(name: String, from: String, output: String) -> Self {
        Self { name, from, output }
    }
}

#[derive(Debug, Clone)]
pub enum StepResult {
    AsyncJob(JobId),
    Completed(StepOutputs),
    ProcessEnded(StepOutputs),
}

#[derive(Debug, Clone)]
pub enum StepExecutionStatus {
    Started,
    Waiting,
    Completed,
}

#[derive(Debug, Clone)]
pub struct StepState {
    pub step_id: String,
    pub status: StepExecutionStatus,
    pub inputs: Map<String, Value>,
    pub outputs: Map<String, Value>,
}

impl StepState {
    pub fn new(step_id: String) -> Self {
        Self {
            step_id,
            status: StepExecutionStatus::Started,
            inputs: Map::default(),
            outputs: Map::default(),
        }
    }
}

pub trait Step: Send + Sync {
    fn id(&self) -> String;

    fn input_schema(&self) -> Option<String> {
        None
    }

    fn output_schema(&self) -> Option<String> {
        None
    }

    fn get_input_requests(&self) -> Vec<StepInputRequest> {
        vec![]
    }

    fn get_next_steps(&self, _ctx: &dyn ManageStep, next_steps: &Vec<String>) -> Vec<String> {
        next_steps.clone()
    }

    fn start(&self, _ctx: &dyn ManageStep) -> Result<StepResult> {
        Ok(StepResult::Completed(Map::default()))
    }
}
