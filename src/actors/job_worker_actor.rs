use actix::{Actor, Handler, Message};

#[derive(Debug, Clone)]
pub struct JobItem {
    pub inputs: String,
    pub job_name: String,
}

impl JobItem {
    pub fn new(inputs: String, job_name: String) -> Self {
        Self { inputs, job_name }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddWorkItem(pub JobItem);

#[derive(Message)]
#[rtype(result = "Vec<JobItem>")]
pub struct GetWorkItems;

#[derive(Debug)]
pub struct JobWorkerActor {
    pub work_items: Vec<JobItem>,
}

impl Default for JobWorkerActor {
    fn default() -> Self {
        JobWorkerActor {
            work_items: Vec::new(),
        }
    }
}

impl Actor for JobWorkerActor {
    type Context = actix::Context<Self>;
}

impl Handler<AddWorkItem> for JobWorkerActor {
    type Result = ();

    fn handle(&mut self, msg: AddWorkItem, _ctx: &mut Self::Context) -> Self::Result {
        self.work_items.push(msg.0);
    }
}

impl Handler<GetWorkItems> for JobWorkerActor {
    type Result = Vec<JobItem>;

    fn handle(&mut self, _msg: GetWorkItems, _ctx: &mut Self::Context) -> Self::Result {
        self.work_items.clone()
    }
}
