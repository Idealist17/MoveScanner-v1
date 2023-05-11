use crate::move_model::stackless_bytecode_generator::*;

#[test]
fn test_source2stackless_ir() {
    let path = "/Users/lteng/Movebit/detect";
    // let path = "/Users/lteng/Movebit/Audit/suipad-contract";
    let pipe_list = "data_invariant_instrumentation";
    let (env, targets) = source2stackless_ir(path, pipe_list);
    let mut text = String::new();
    text += &print_targets_for_test(&env, "initial translation from Move", &targets);
    println!("{}", text);
    // get_stackless_bytecode(&env, &targets);

    // use std::io::Write;
    // let mut file = std::fs::File::create("data1.txt").expect("create failed");
    // file.write_all(text.as_bytes()).expect("write failed");
    // println!("data written to file" );
}

#[test]
fn test_get_from_bytecode_modules() {
    let dir = "/Users/lteng/Movebit/detect/build/movebit/bytecode_modules";
    let (env, targets) = get_from_bytecode_modules(dir);
    let mut text = String::new();
    text += &print_targets_for_test(&env, "initial translation from Move", &targets);
    let functions = get_stackless_bytecode(&env, &targets);
}