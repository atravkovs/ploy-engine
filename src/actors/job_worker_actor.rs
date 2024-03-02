use actix::{Actor, Handler, Message, Recipient};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobStatus {
    Open,
    InProgress,
    Completed,
}

#[derive(Debug, Clone)]
pub struct JobItem {
    pub id: String,
    pub inputs: String,
    pub job_name: String,
    pub status: JobStatus,
}

impl JobItem {
    pub fn new(id: String, inputs: String, job_name: String) -> Self {
        Self {
            id,
            inputs,
            job_name,
            status: JobStatus::Open,
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddWorkItem(pub JobItem);

#[derive(Message, Clone, Debug)]
#[rtype(result = "Result<(), anyhow::Error>")]
pub struct JobCompletedMessage {
    pub job_id: String,
}

impl JobCompletedMessage {
    pub fn new(job_id: String) -> Self {
        Self { job_id }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddCompletionSubscriber(pub Recipient<JobCompletedMessage>);

#[derive(Message)]
#[rtype(result = "Vec<JobItem>")]
pub struct GetWorkItems;

#[derive(Debug)]
pub struct JobWorkerActor {
    pub work_items: Vec<JobItem>,
    pub completed_subscribers: Vec<Recipient<JobCompletedMessage>>,
}

impl JobWorkerActor {
    pub fn new() -> Self {
        JobWorkerActor {
            work_items: Vec::new(),
            completed_subscribers: Vec::new(),
        }
    }

    fn set_in_progress(&mut self, id: &str) {
        if let Some(work_item) = self
            .work_items
            .iter_mut()
            .find(|work_item| work_item.id == id)
        {
            work_item.status = JobStatus::InProgress;
        }
    }
}

impl Default for JobWorkerActor {
    fn default() -> Self {
        JobWorkerActor::new()
    }
}

impl Actor for JobWorkerActor {
    type Context = actix::Context<Self>;
}

impl Handler<AddWorkItem> for JobWorkerActor {
    type Result = ();

    fn handle(&mut self, msg: AddWorkItem, _ctx: &mut Self::Context) -> Self::Result {
        println!("Adding work item: {:?}", msg.0);
        self.work_items.push(msg.0);
    }
}

impl Handler<GetWorkItems> for JobWorkerActor {
    type Result = Vec<JobItem>;

    fn handle(&mut self, _msg: GetWorkItems, _ctx: &mut Self::Context) -> Self::Result {
        let items = self
            .work_items
            .iter()
            .filter(|work_item| work_item.status == JobStatus::Open)
            .take(2)
            .cloned()
            .collect::<Vec<JobItem>>();

        items
            .iter()
            .for_each(|work_item| self.set_in_progress(&work_item.id));

        items
    }
}

impl Handler<AddCompletionSubscriber> for JobWorkerActor {
    type Result = ();

    fn handle(&mut self, msg: AddCompletionSubscriber, _ctx: &mut Self::Context) -> Self::Result {
        self.completed_subscribers.push(msg.0);
    }
}

impl Handler<JobCompletedMessage> for JobWorkerActor {
    type Result = Result<()>;

    fn handle(&mut self, msg: JobCompletedMessage, _ctx: &mut Self::Context) -> Self::Result {
        // TODO optimize this
        self.work_items
            .iter_mut()
            .find(|work_item| work_item.id == msg.job_id)
            .map(|work_item| work_item.status = JobStatus::Completed);

        self.completed_subscribers
            .iter()
            .for_each(|sub| sub.do_send(msg.clone()));

        Ok(())
    }
}
