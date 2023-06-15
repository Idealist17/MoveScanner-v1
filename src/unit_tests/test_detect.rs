use std::path::PathBuf;
use std::str::FromStr;
use crate::move_ir::{
    bytecode_display, 
    generate_bytecode::StacklessBytecodeGenerator
};
use crate::utils::utils::compile_module;
use crate::detect::detect1::detect_unchecked_return;


#[test]
fn test_detect_unchecked_return() {
    let filename = PathBuf::from_str("/Users/lteng/Movebit/detect/build/movebit/bytecode_modules/unchecked_return.mv").unwrap();
    let cm = compile_module(filename);
    for fd in &cm.function_defs {
        let mut stbgr = StacklessBytecodeGenerator::new(&cm, fd);
        stbgr.generate_function();
        for function in stbgr.functions.iter() {
            if detect_unchecked_return(function) {
                println!("{}", "111");
            }
        }
    }
}
