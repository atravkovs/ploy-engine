use anyhow::Result;
use std::{fs, rc::Rc};

mod parser;
mod step;

fn execute_step(step: &Rc<dyn step::Step>) {
    step.execute();

    let next_steps = step.next();

    for next_step in next_steps.iter() {
        execute_step(next_step);
    }
}

fn main() -> Result<()> {
    let file_name = "Test.ploy";
    let file_contents = fs::read_to_string(file_name)?;

    let start_step = parser::parse_xml(&file_contents)?;

    execute_step(&start_step);

    Ok(())
}
