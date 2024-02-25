use std::{cell::RefCell, rc::Rc};

use actix::{Actor, Addr, AsyncContext, Handler};
use serde_json::{Map, Value};
use uuid::Uuid;

use crate::engine::{
    step::{self, StepLeaf},
    ProcessContext, Step, StepStatus,
};

use super::job_worker_actor::{JobCompletedMessage, JobItem, JobWorkerActor};

struct StartStep {
    id: String,
    activity_id: String,
}

impl StartStep {
    fn new(activity_id: String) -> Self {
        let id = Uuid::new_v4().to_string();

        Self { id, activity_id }
    }
}

impl Step for StartStep {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn activity_id(&self) -> String {
        self.activity_id.clone()
    }
}

struct EndStep {
    id: String,
    activity_id: String,
}

impl EndStep {
    fn new(activity_id: String) -> Self {
        let id = Uuid::new_v4().to_string();

        Self { id, activity_id }
    }
}

impl Step for EndStep {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn activity_id(&self) -> String {
        self.activity_id.clone()
    }
}

pub struct ActivityStep {
    id: String,
    activity_id: String,
    status: StepStatus,
    inputs: Map<String, Value>,
    job_name: String,
}

impl ActivityStep {
    pub fn new(activity_id: String, inputs: Map<String, Value>, job_name: String) -> Self {
        let id = Uuid::new_v4().to_string();

        Self {
            id,
            activity_id,
            inputs,
            job_name,
            status: StepStatus::Open,
        }
    }
}

impl Step for ActivityStep {
    fn start(&mut self, ctx: &mut ProcessContext) {
        self.status = StepStatus::InProgress;

        ctx.add_job(JobItem::new(
            self.id.clone(),
            Value::Object(self.inputs.clone()).to_string(),
            self.job_name.clone(),
        ));
    }

    fn end(&mut self, _ctx: &mut ProcessContext) {
        self.status = StepStatus::Completed;
    }

    fn status(&self) -> StepStatus {
        self.status.clone()
    }

    fn id(&self) -> String {
        self.id.clone()
    }

    fn activity_id(&self) -> String {
        self.activity_id.clone()
    }
}

fn map_step(leaf: &StepLeaf) -> Rc<RefCell<dyn Step>> {
    match leaf.step() {
        step::ActivityType::StartNode => Rc::new(RefCell::new(StartStep::new(leaf.id.clone()))),
        step::ActivityType::EndNode => Rc::new(RefCell::new(EndStep::new(leaf.id.clone()))),
        step::ActivityType::ActivityNode(activity) => {
            let mut inputs = Map::default();
            activity.inputs.iter().for_each(|(k, v)| {
                inputs.insert(k.clone(), Value::String(v.clone()));
            });

            Rc::new(RefCell::new(ActivityStep::new(
                leaf.id.clone(),
                inputs,
                activity.job.clone(),
            )))
        }
    }
}

pub struct ProcessActor {
    job_worker: Addr<JobWorkerActor>,
    ctx: ProcessContext,
    queue: Vec<Rc<RefCell<dyn Step>>>,
    in_progress_queue: Vec<Rc<RefCell<dyn Step>>>,
    process_definition: Option<StepLeaf>,
}

impl ProcessActor {
    pub fn new(job_worker: Addr<JobWorkerActor>) -> Self {
        let queue = Vec::default();
        let in_progress_queue = Vec::default();

        let ctx = ProcessContext::new(job_worker.clone());

        Self {
            job_worker,
            ctx,
            queue,
            in_progress_queue,
            process_definition: Option::None,
        }
    }

    fn process_queue(&mut self) {
        if let Some(step) = self.queue.pop() {
            step.borrow_mut().start(&mut self.ctx);
            self.check_completion(step);
        }
    }

    fn check_completion(&mut self, step_ref: Rc<RefCell<dyn Step>>) {
        if step_ref.borrow().status() == StepStatus::Completed {
            println!("Step {} completed", step_ref.borrow().id());
            let next = self
                .process_definition
                .as_ref()
                .unwrap()
                .next_for(&step_ref.borrow().activity_id());
            for next in next {
                self.queue.push(map_step(&next.borrow()));
            }
            self.process_queue();
        } else {
            self.in_progress_queue.push(step_ref);
        }
    }
}

impl Actor for ProcessActor {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.job_worker
            .do_send(crate::actors::job_worker_actor::AddCompletionSubscriber(
                ctx.address().recipient(),
            ));

        let file_name = "data/Test.ploy";
        let file_contents = std::fs::read_to_string(file_name).unwrap();

        self.process_definition =
            Option::Some(crate::engine::parser::parse_xml(&file_contents).unwrap());

        let step = map_step(&self.process_definition.as_ref().unwrap());
        self.queue.push(step);
        self.process_queue();
    }
}

impl Handler<JobCompletedMessage> for ProcessActor {
    type Result = ();

    fn handle(&mut self, msg: JobCompletedMessage, _ctx: &mut Self::Context) -> Self::Result {
        let step = self
            .in_progress_queue
            .iter()
            .find(|step| step.borrow().id() == msg.step_id);

        if let Some(step) = step {
            step.borrow_mut().end(&mut self.ctx);

            self.queue.retain(|step| step.borrow().id() == msg.step_id);
            self.check_completion(step.clone());
        }
    }
}
