use std::collections::HashMap;

use rustpython::{
    self,
    vm::{
        self,
        builtins::{PyDict, PyMap, PyStr},
        convert::ToPyObject,
        stdlib, PyRef, Settings,
    },
};
use serde_json::Map;

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
        println!("Hello from script step!");

        // TODO! - Set python library path from environment variable + bundle w docker image
        let mut settings: Settings =
            (Settings::default()).with_path("/home/s0ck3t/ploy/RustPython/Lib".to_string());
        settings.no_sig_int = true;
        settings.debug = true;
        settings.inspect = true;

        let interpreter = rustpython::vm::Interpreter::with_init(settings, |vm| {
            vm.add_native_modules(stdlib::get_module_inits());
            vm.add_native_modules(rustpython_stdlib::get_module_inits());
        });

        interpreter
            .enter(|vm| -> vm::PyResult<()> {
                vm.insert_sys_path(vm.new_pyobj("data/python"))
                    .expect("add path");

                let module_res = vm.import("test", None, 0);

                if let Err(err) = module_res {
                    vm.print_exception(err);

                    return Ok(());
                }

                let module = module_res.expect("Module imported");

                let take_string_fn = module.get_attr("take_string", vm).expect("get take string");

                take_string_fn
                    .call((String::from("Rust str -> Python"),), vm)
                    .expect("call take string");

                Ok(())
            })
            .map_err(|err| anyhow::anyhow!("Error: {:?}", err))?;

        Ok(crate::definition::step::StepResult::Completed(
            Map::default(),
        ))
    }
}
