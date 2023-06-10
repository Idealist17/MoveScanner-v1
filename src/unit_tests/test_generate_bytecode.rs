use std::path::PathBuf;
use std::str::FromStr;

use crate::move_ir::generate_bytecode::*;
use crate::utils::utils::compile_module;
use move_stackless_bytecode::stackless_bytecode::Bytecode;
use crate::move_ir::bytecode_display;

#[test]
fn test_generate_bytecode() {
    let filename = PathBuf::from_str("/Users/lteng/Movebit/detect/build/movebit/bytecode_modules/witness_user.mv").unwrap();
    let cm = compile_module(filename);
    for fd in &cm.function_defs { 
        let mut bg = StacklessBytecodeGenerator::new(&cm, fd);
        bg.generate_function();
        let bytecodes = bg.code.clone();
        println!("{}", bytecodes.len());
        let label_offsets = Bytecode::label_offsets(&bytecodes);

        for (offset, code) in bytecodes.iter().enumerate() {
            println!(
                "{}",
                format!("{:>3}: {}", offset, bytecode_display::display(code, &label_offsets, &bg))
            );
        }
    }
}