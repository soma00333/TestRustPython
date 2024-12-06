use rustpython_stdlib;
use rustpython_vm::{self as vm};
use serde_json::{json, Value};
use std::fs;
fn main() -> vm::PyResult<()> {
    let source = fs::read_to_string("script.py").expect("Failed to read the Python script file");

    vm::Interpreter::with_init(Default::default(), |vm| {
        vm.add_native_modules(rustpython_stdlib::get_module_inits());
    })
    .enter(|vm| {
        let scope = vm.new_scope_with_builtins();

        if let Err(err) = vm.run_code_string(scope.clone(), &source, "<script>".to_owned()) {
            vm.print_exception(err.clone());
            return Err(err);
        }

        let json_data = json!({"key": "value", "number": 42}).to_string();
        let py_json_str = vm.ctx.new_str(json_data);

        let process_func = scope.globals.get_item("process", vm)?;

        let output_data = process_func.call((py_json_str,), vm)?;
        let output_str: String = output_data.try_into_value(vm)?;

        let parsed_output: Value = serde_json::from_str(&output_str).unwrap();
        println!("Python returned: {:?}", parsed_output);

        Ok(())
    })
}

// fn main() -> vm::PyResult<()> {
//     let source = fs::read_to_string("script.py").expect("Failed to read the Python script file");
//     vm::Interpreter::with_init(Default::default(), |vm| {
//         vm.add_native_modules(rustpython_stdlib::get_module_inits());
//     })
//     .enter(|vm| {
//         let scope = vm.new_scope_with_builtins();
//         let code_obj = vm
//             .compile(&source, vm::compiler::Mode::Exec, "<embedded>".to_owned())
//             .map_err(|err| vm.new_syntax_error(&err, Some(&source)))?;

//         vm.run_code_obj(code_obj, scope)?;

//         Ok(())
//     })
// }
