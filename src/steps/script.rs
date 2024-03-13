use rustpython::{
    self,
    vm::{self, stdlib, Settings},
};
use serde_json::{json, value::Serializer, Map};
use vm::py_serde::{deserialize, serialize};

use crate::definition::step::Step;

#[derive(Debug, Clone)]
pub struct ScriptStep {
    pub id: String,
}

impl ScriptStep {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl Step for ScriptStep {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn start(
        &self,
        _ctx: &dyn crate::definition::step::ManageStep,
    ) -> anyhow::Result<crate::definition::step::StepResult> {
        // TODO! - Set python library path from environment variable + bundle w docker image
        let mut settings: Settings =
            (Settings::default()).with_path("/home/s0ck3t/ploy/RustPython/Lib".to_string());
        settings.no_sig_int = true;
        settings.debug = true;
        settings.inspect = true;

        let sample_input_json = json!({ "hello": "world!" });

        let interpreter = rustpython::vm::Interpreter::with_init(settings, |vm| {
            vm.add_native_modules(stdlib::get_module_inits());
            vm.add_native_modules(rustpython_stdlib::get_module_inits());
        });

        interpreter.enter(|vm| {
            vm.insert_sys_path(vm.new_pyobj("data/python"))
                .expect("add path");
            vm.import("pre-import", None, 0).expect("Pre-import works");
        });

        interpreter
            .enter(|vm| -> vm::PyResult<()> {
                let module_res = vm.import("test", None, 0);

                if let Err(err) = module_res {
                    vm.print_exception(err);

                    return Ok(());
                }

                let module = module_res.expect("Module imported");

                let execute_fn = module
                    .get_attr("execute", vm)
                    .expect("Should have execute function");

                let py_input =
                    deserialize(vm, sample_input_json).expect("Convert JSON to PyObject");

                let execution_result = execute_fn.call((py_input,), vm);

                if let Err(err) = execution_result {
                    vm.print_exception(err);

                    return Ok(());
                }

                let result = execution_result.expect("Execution result");

                let result_json =
                    serialize(vm, &result, Serializer).expect("Convert PyObject to JSON");

                println!("Result: {}", result_json);

                Ok(())
            })
            .map_err(|err| anyhow::anyhow!("Error: {:?}", err))?;

        Ok(crate::definition::step::StepResult::Completed(
            Map::default(),
        ))
    }
}
