use crate::definition::step::Step;

pub struct EndStep {
    pub id: String,
}

impl EndStep {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl Step for EndStep {
    fn id(&self) -> String {
        self.id.clone()
    }
}
