// precision_loss

use std::rc::Rc;

use crate::move_ir::generate_bytecode::FunctionInfo;
use move_model::symbol::SymbolPool;
use move_stackless_bytecode::stackless_bytecode::{
    Bytecode, Operation
};


pub fn detect_precision_loss(function: &FunctionInfo, symbol_pool: &SymbolPool) -> bool {
    let mut ret_flag = false;
    for (code_offset, bytecode) in function.code.iter().enumerate() {
        match &bytecode {
            Bytecode::Call(_, _, Operation::Mul, srcs, _) => {
                // println!("{:?}",bytecode);
                let oprand1 = get_oprand_bytecode(&function.code, code_offset-1, srcs[0]);
                let oprand2 = get_oprand_bytecode(&function.code, code_offset, srcs[1]);
                // println!("{:?}", oprand1);
                // println!("{:?}", oprand2);
                if is_div(oprand1) || is_div(oprand2) || is_sqrt(oprand1, symbol_pool) || is_sqrt(oprand2, symbol_pool) {
                    ret_flag = true;
                    break;
                }
            },
            _ => {
                continue;
            }
        }
    }
    ret_flag 
}

fn get_oprand_bytecode(bytecodes: &Vec<Bytecode>, code_offset: usize, src_idx: usize) -> &Bytecode {
    let mut tmp_index = code_offset - 1;
    while tmp_index!=0 {
        match &bytecodes[tmp_index] {
            Bytecode::Call(_, dst, _, _, _) => {
                if dst[0] == src_idx  {
                    return &bytecodes[tmp_index];
                } else {
                    tmp_index = tmp_index - 1;
                    continue;
                }
            },
            Bytecode::Assign(_, dst, _, _) => {
                if *dst == src_idx {
                    return &bytecodes[tmp_index];
                } else {
                    tmp_index = tmp_index - 1;
                    continue;
                }
            },
            Bytecode::Load(_, dst, _) => {
                if *dst == src_idx {
                    return &bytecodes[tmp_index];
                } else {
                    tmp_index = tmp_index - 1;
                    continue;
                }
            },
            _ => {
                tmp_index = tmp_index - 1;
                continue;
            }
        }
    }
    return &bytecodes[tmp_index];
}

fn is_div(bytecode: &Bytecode) -> bool {
    let mut ret_flag = false;
    match bytecode {
        Bytecode::Call(_, _, Operation::Div, _, _) => {
            ret_flag = true;
        },
        _ => {
        }
    }
    return ret_flag;
}

fn is_sqrt(bytecode: &Bytecode, symbol_pool: &SymbolPool) -> bool {
    let mut ret_flag = false;
    match bytecode {
        Bytecode::Call(_, _, Operation::Function(_, funid, _), _, _) => {
            if symbol_pool.string(funid.symbol()) == Rc::from("sqrt".to_string()) {
                ret_flag = true;
            }
        },
        _ => {
        }
    }
    return ret_flag;
}