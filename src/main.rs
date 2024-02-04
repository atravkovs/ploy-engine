use anyhow::Result;
use std::{collections::HashMap, fs, rc::Rc};

mod parser;
mod step;

#[derive(Debug, Clone, Default)]
pub struct ProcessContext {
    state: HashMap<String, String>,
}

impl ProcessContext {
    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.state.get(name)
    }

    pub fn set_variable(&mut self, name: String, value: String) {
        self.state.insert(name, value);
    }
}

fn execute_step(step: &Rc<dyn step::Step>, ctx: &mut ProcessContext) {
    step.execute(ctx);

    let next_steps = step.next();

    for next_step in next_steps.iter() {
        execute_step(next_step, ctx);
    }
}

fn main() -> Result<()> {
    let file_name = "data/Test.ploy";
    let file_contents = fs::read_to_string(file_name)?;

    let start_step = parser::parse_xml(&file_contents)?;

    let mut ctx = ProcessContext::default();
    execute_step(&start_step, &mut ctx);

    Ok(())
}
