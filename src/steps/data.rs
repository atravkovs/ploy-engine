use crate::definition::step::Step;

pub struct DataStep {
    pub id: String,
}

impl DataStep {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl Step for DataStep {
    fn id(&self) -> String {
        self.id.clone()
    }
}
