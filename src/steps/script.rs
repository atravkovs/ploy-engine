use rustpython::{
    self,
    vm::{self, stdlib, Settings},
};
use serde_json::{value::Serializer, Map, Value};
use vm::py_serde::{deserialize, serialize};

use crate::definition::step::{Step, StepInputRequest};

#[derive(Debug, Clone)]
pub struct ScriptStep {
    id: String,
    script: String,
    input_schema: String,
    output_schema: String,
    inputs: Vec<StepInputRequest>,
}

impl ScriptStep {
    pub fn new(
        id: String,
        script: String,
        input_schema: String,
        output_schema: String,
        inputs: Vec<StepInputRequest>,
    ) -> Self {
        Self {
            id,
            script,
            inputs,
            input_schema,
            output_schema,
        }
    }
}

impl Step for ScriptStep {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn get_input_requests(&self) -> Vec<StepInputRequest> {
        self.inputs.clone()
    }

    fn input_schema(&self) -> Option<String> {
        Some(self.input_schema.clone())
    }

    fn output_schema(&self) -> Option<String> {
        Some(self.output_schema.clone())
    }

    fn start(
        &self,
        ctx: &dyn crate::definition::step::ManageStep,
    ) -> anyhow::Result<crate::definition::step::StepResult> {
        // TODO! - Set python library path from environment variable + bundle w docker image
        let mut settings: Settings =
            (Settings::default()).with_path("./Lib".to_string());
        settings.no_sig_int = true;
        settings.debug = true;
        settings.inspect = true;

        let inputs = Value::Object(ctx.get_inputs().clone());

        let interpreter = rustpython::vm::Interpreter::with_init(settings, |vm| {
            vm.add_native_modules(stdlib::get_module_inits());
            vm.add_native_modules(rustpython_stdlib::get_module_inits());
        });

        interpreter.enter(|vm| {
            vm.insert_sys_path(vm.new_pyobj("data/python"))
                .expect("add path");
            vm.import("pre-import", None, 0).expect("Pre-import works");
        });

        let script_result = interpreter
            .enter(|vm| -> vm::PyResult<Value> {
                let module_name = vm.ctx.new_str(self.script.clone());

                let module_res = vm.import(&module_name, None, 0);

                if let Err(err) = module_res {
                    vm.print_exception(err);

                    return Ok(Value::Null);
                }

                let module = module_res.expect("Module imported");

                let execute_fn = module
                    .get_attr("execute", vm)
                    .expect("Should have execute function");

                let py_input = deserialize(vm, inputs).expect("Convert JSON to PyObject");

                let execution_result = execute_fn.call((py_input,), vm);

                if let Err(err) = execution_result {
                    vm.print_exception(err);

                    return Ok(Value::Null);
                }

                let result = execution_result.expect("Execution result");

                let result_json =
                    serialize(vm, &result, Serializer).expect("Convert PyObject to JSON");

                Ok(result_json)
            })
            .expect("All is okay");

        if let Value::Object(result) = script_result {
            return Ok(crate::definition::step::StepResult::Completed(result));
        } else {
            // return Ok(crate::definition::step::StepResult::Failed(
            //     "Script did not return a JSON object".to_string(),
            // ));

            Ok(crate::definition::step::StepResult::Completed(
                Map::default(),
            ))
        }
    }

    fn get_type(&self) -> crate::definition::step::StepType {
        crate::definition::step::StepType::ScriptStep
    }
}
