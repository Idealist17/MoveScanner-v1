// unckecked_return

use crate::move_ir::generate_bytecode::FunctionInfo;
use move_stackless_bytecode::stackless_bytecode::{
    Bytecode, Operation
};


pub fn detect_unchecked_return(function: &FunctionInfo) -> bool {
    let mut res = vec![];
    for (code_offset, bytecode) in function.code.iter().enumerate() {
        match &bytecode {
            Bytecode::Call(_, dsts , Operation::Function(_, _, _), _, _) => {
                let ret_cnt = dsts.len();
                // 函数没有返回值 false
                let mut flag = if ret_cnt == 0 { false } else { true } ;
                for (id , dst) in dsts.iter().enumerate() {
                    // 从后向前依次 pop(destory) 掉函数返回值，对每一个返回值进行检测
                    match &function.code[code_offset+ret_cnt-id] {
                        Bytecode::Call(_, _, Operation::Destroy, destory_srcs, _) => {
                            if destory_srcs[0] == *dst {
                                continue;
                            } else {
                                flag = false;
                                break;
                            }
                        },
                        _ => {
                            flag = false;
                            break;
                        }
                    }
                }
                res.push(flag);
            },
            _ => {
                continue;
            }
        }
    }
    res.contains(&true)
}
