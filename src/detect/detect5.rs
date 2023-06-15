// // unused_constant

// use crate::move_ir::generate_bytecode::StacklessBytecodeGenerator;
// use move_binary_format::access::ModuleAccess;
// use move_stackless_bytecode::stackless_bytecode::{
//     Bytecode, Operation
// };


// pub fn detect_unused_constants(stbgr: &StacklessBytecodeGenerator) {
//     let aa = stbgr.module.constant_pool();

//     let md = stbgr.module_data.named_constants;
//     for (name, constant) in stbgr.module_data.named_constants.into_iter() {

//     }
//     // let cm = &module.module;
//     // let const_pool = &cm.constant_pool;
//     // let len = const_pool.len();
//     // let mut used = vec![false; len];
//     // for fun in cm.function_defs.iter() {
//     //     if let Some(codes) = &fun.code {
//     //         for code in codes.code.iter() {
//     //             match code {
//     //                 MoveBytecode::LdConst(idx) => {
//     //                     used[idx.into_index()] = true;
//     //                 },
//     //                 _ => {},
//     //             }
//     //         };
//     //     } else {
//     //         continue;
//     //     }
//     // }
//     // println!("{:?}", used);
// }