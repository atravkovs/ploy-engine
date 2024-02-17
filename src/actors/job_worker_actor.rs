use actix::{Actor, Handler, Message};

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddWorkItem(pub String);

#[derive(Message)]
#[rtype(result = "Vec<String>")]
pub struct GetWorkItems;

#[derive(Debug)]
pub struct JobWorkerActor {
    pub work_items: Vec<String>,
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
    type Result = Vec<String>;

    fn handle(&mut self, _msg: GetWorkItems, _ctx: &mut Self::Context) -> Self::Result {
        self.work_items.clone()
    }
}
