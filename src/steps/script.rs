use rustpython::{
    self,
    vm::{self, Settings},
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

        let mut settings: Settings = Default::default();
        settings.no_sig_int = true;

        let interpreter = rustpython::InterpreterConfig::new()
            .settings(settings)
            .interpreter();

        interpreter
            .enter(|vm| -> vm::PyResult<()> {
                let scope = vm.new_scope_with_builtins();
                let source = r#"print("Hello World!")"#;

                let code_obj = vm
                    .compile(source, vm::compiler::Mode::Exec, "<embedded>".to_owned())
                    .map_err(|err| vm.new_syntax_error(&err, Some(source)))?;

                vm.run_code_obj(code_obj, scope)?;

                Ok(())
            })
            .map_err(|err| anyhow::anyhow!("Error: {:?}", err))?;

        Ok(crate::definition::step::StepResult::Completed(
            Map::default(),
        ))
    }
}
