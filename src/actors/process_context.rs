use core::fmt;

use actix::Addr;

use super::process_actor::ProcessActor;

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessState {
    Running,
    Completed,
    Failed,
}

impl fmt::Display for ProcessState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessState::Running => write!(f, "Running"),
            ProcessState::Completed => write!(f, "Completed"),
            ProcessState::Failed => write!(f, "Failed"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProcessContext {
    pub process_id: String,
    pub process_addr: Option<Addr<ProcessActor>>,
    pub state: ProcessState,
    pub outputs: Option<serde_json::Map<String, serde_json::Value>>,
}

impl ProcessContext {
    pub fn new(process_id: String, process_addr: Addr<ProcessActor>) -> Self {
        Self {
            process_id,
            process_addr: Some(process_addr),
            state: ProcessState::Running,
            outputs: None,
        }
    }
}
