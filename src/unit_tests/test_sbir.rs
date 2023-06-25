#![allow(unused_imports)]
use crate::move_ir::sbir_generator::*;

#[test]
fn test_source2stackless_ir() {
    let path = "/Users/lteng/Movebit/detect";
    let pipe_list = "data_invariant_instrumentation";
    let (_, _) = source2stackless_ir(path, pipe_list);
}


// need all modules, so use dir as input
#[test]
fn test_get_from_bytecode_modules() {
    let dir = "./testdata/examples_mv/aptos/";
    let bc = Blockchain::Aptos;
    let ms = MoveScanner::new(dir, bc);

    let mut text = String::new();
    text += &ms.print_targets_for_test();
    // println!("{}", text);

    use std::io::Write;
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all(text.as_bytes()).expect("write failed");
}