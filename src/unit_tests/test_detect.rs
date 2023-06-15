use std::path::PathBuf;
use std::str::FromStr;
use move_binary_format::access::ModuleAccess;
use move_binary_format::file_format::FunctionDefinitionIndex;

use crate::move_ir::{
    bytecode_display, 
    generate_bytecode::StacklessBytecodeGenerator
};
use crate::utils::utils::compile_module;
use crate::detect::{
    detect1::detect_unchecked_return,
    detect5::detect_unused_constants,
};



#[test]
fn test_detect_unchecked_return() {
    let filename = PathBuf::from_str("testdata/examples_mv/aptos/unchecked_return.mv").unwrap();
    let cm = compile_module(filename);
    let mut stbgr = StacklessBytecodeGenerator::new(&cm);
    stbgr.generate_function();
    for function in stbgr.functions.iter() {
        if detect_unchecked_return(function) {
            println!("{}", "111");
        }
    }
}
