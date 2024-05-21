use std::{collections::HashMap, sync::Arc};

use log::warn;

use super::{process_definition::ProcessDefinition, step::StepInputRequest};

#[derive(Debug, Clone, PartialEq)]
pub struct IODescriptor {
    pub name: String,
    pub rtype: String,
    pub required: bool,
}

impl IODescriptor {
    pub fn new(name: String, rtype: String, required: bool) -> Self {
        IODescriptor {
            name,
            rtype,
            required,
        }
    }
}

pub struct ProcessValidator {
    process_definition: Arc<ProcessDefinition>,
    stack: HashMap<String, bool>,
    visited: HashMap<String, bool>,
}

impl ProcessValidator {
    fn new(process_definition: Arc<ProcessDefinition>) -> Self {
        let stack = HashMap::default();
        let visited = HashMap::default();

        ProcessValidator {
            process_definition,
            stack,
            visited,
        }
    }

    pub fn validate(process_definition: Arc<ProcessDefinition>) -> bool {
        Self::new(process_definition).validate_internal()
    }

    fn validate_internal(&mut self) -> bool {
        let start_step = self.process_definition.get_start_step_id();

        self.validate_step(&start_step)
    }

    fn validate_step(&mut self, step_id: &str) -> bool {
        if self.is_in_stack(step_id) {
            warn!("Cycle detected: {}", step_id);
            return false;
        }

        if self.was_already_visited(step_id) {
            return true;
        }

        self.stack.insert(step_id.to_string(), true);
        self.visited.insert(step_id.to_string(), true);

        let step = self.process_definition.get_step(step_id).unwrap();
        let next_steps = self.process_definition.get_next(step_id);

        let input_requests = step.get_input_requests();

        if !self.check_missing_required_input_requests(step_id) {
            return false;
        }

        for input_request in input_requests {
            if !self.check_input_request_conformance(step_id, &input_request) {
                return false;
            }
        }

        if next_steps.is_none() || next_steps.unwrap().is_empty() {
            if !step.get_type().is_end() {
                warn!("Last process step is not End: {}", step_id);
                return false;
            }

            return true;
        }

        let next_steps = next_steps
            .expect("Next steps should exist at this point")
            .iter()
            .map(|next_step| next_step.to.clone())
            .collect::<Vec<String>>();

        for next_step in next_steps {
            if !self.validate_step(&next_step) {
                return false;
            }
        }

        self.stack.insert(step_id.to_string(), false);

        true
    }

    fn check_missing_required_input_requests(&self, step_id: &str) -> bool {
        let step = self.process_definition.get_step(step_id).unwrap();
        let input_requests = step.get_input_requests();

        let input_schema = step.input_schema();

        if input_schema.is_none() {
            warn!("Step {} does not have input schema", step_id);

            // TODO! Rework when there will be support for Process IO schemas
            return true;
        }

        let input_schema = input_schema.unwrap();
        let input_schema = Self::load_schema(&input_schema);

        input_schema["required"]
            .as_array()
            .map(|required| {
                for required_field in required {
                    let required_field = required_field.as_str().unwrap();

                    if !input_requests
                        .iter()
                        .any(|input_request| input_request.name == required_field)
                    {
                        warn!(
                            "Required input field '{}' is missing in mapping of step {}",
                            required_field, step_id
                        );
                        return false;
                    }
                }

                true
            })
            .unwrap_or(true)
    }

    fn is_in_stack(&self, step_id: &str) -> bool {
        self.stack.get(step_id).is_some_and(|item| item == &true)
    }

    fn was_already_visited(&self, step_id: &str) -> bool {
        self.visited.get(step_id).is_some_and(|item| item == &true)
    }

    // TODO? Optimize schema loading
    fn check_input_request_conformance(&self, step_id: &str, request: &StepInputRequest) -> bool {
        let step = self
            .process_definition
            .get_step(&request.from)
            .expect("Input should be mapped from existing step");

        if step.get_type().is_flow_step() && !self.is_in_stack(&request.from) {
            warn!(
                "Step {} should be executed before {} to map inputs from it",
                request.from, step_id
            );
            return false;
        }

        let output_schema = step.output_schema();

        if output_schema.is_none() {
            warn!("Step {} does not have output schema", request.from);

            // TODO! Rework when there will be support for Process IO schemas
            return true;
        }

        let output_schema = output_schema.unwrap();
        let output_schema = Self::load_schema(&output_schema);

        let request_step = self
            .process_definition
            .get_step(step_id)
            .expect("Step exists");

        let input_schema = request_step.input_schema();

        if input_schema.is_none() {
            warn!("Step {} does not have input schema", step_id);

            // TODO! Rework when there will be support for Process IO schemas
            return true;
        }

        let input_schema = input_schema.unwrap();
        let input_schema = Self::load_schema(&input_schema);

        let output_field = Self::get_field(&output_schema, &request.output);
        let input_field = Self::get_field(&input_schema, &request.name);

        if output_field.is_none() {
            warn!(
                "Output field '{}' does not exist in schema {}",
                request.output, output_schema
            );

            return false;
        }

        if input_field.is_none() {
            warn!(
                "Input field '{}' does not exist in schema {}",
                request.name, input_schema
            );

            return false;
        }

        let output_field = output_field.unwrap();
        let input_field = input_field.unwrap();

        if output_field.rtype != input_field.rtype {
            warn!(
                "Output field '{}' type {} does not match input field {} type {}",
                request.output, output_field.rtype, request.name, input_field.rtype
            );

            return false;
        }

        if input_field.required && !output_field.required {
            warn!(
                "Input field '{}' is required but output field '{}' is not",
                request.name, request.output
            );

            return false;
        }

        true
    }

    fn get_field(schema: &serde_json::Value, field_name: &str) -> Option<IODescriptor> {
        schema["properties"]
            .as_object()
            .and_then(|properties| properties.get(field_name))
            .and_then(|field| {
                let required = schema["required"]
                    .as_array()
                    .map(|required| {
                        required.contains(&serde_json::Value::String(field_name.to_string()))
                    })
                    .unwrap_or(false);

                Some(IODescriptor::new(
                    field_name.to_string(),
                    field["type"].to_string(),
                    required,
                ))
            })
    }

    fn load_schema(schema_name: &str) -> serde_json::Value {
        let schema_contents =
            std::fs::read_to_string(format!("data/schemas/{schema_name}.json")).unwrap();

        return serde_json::from_str(&schema_contents).unwrap();
    }
}
