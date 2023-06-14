use std::path::PathBuf;
use std::str::FromStr;

use crate::move_ir::generate_bytecode::*;
use crate::utils::utils::compile_module;
use move_stackless_bytecode::stackless_bytecode::Bytecode;
use crate::move_ir::bytecode_display;

#[test]
fn test_generate_bytecode() {
    let filename = PathBuf::from_str("/Users/lteng/Library/Containers/com.tencent.xinWeChat/Data/Library/Application Support/com.tencent.xinWeChat/2.0b4.0.9/b9d331b4a8e35aa95826433692d9aa9c/Message/MessageTemp/4e7aa6f1fa7f648d128ea5ad15fb0247/File/router.mv").unwrap();
    let cm = compile_module(filename);
    for fd in &cm.function_defs { 
        let mut bg = StacklessBytecodeGenerator::new(&cm, fd);
        bg.generate_function();
        println!("{}", bg);
    }
}
