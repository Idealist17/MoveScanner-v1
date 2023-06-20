// infinite_loop

use std::collections::{BTreeSet, btree_set::Union};

use move_compiler::shared::ast_debug::print;
use move_stackless_bytecode::{stackless_control_flow_graph::BlockContent, stackless_bytecode::Bytecode};

use crate::move_ir::{generate_bytecode::{FunctionInfo, StacklessBytecodeGenerator}, fatloop::get_loops, data_dependency::data_dependency, control_flow_graph::BlockId};


pub fn detect_infinite_loop(stbgr: &StacklessBytecodeGenerator, idx: usize) -> bool {
    let function = &stbgr.functions[idx];
    let (natural_loops, fat_loops) = get_loops(function);
    let data_depent = data_dependency(stbgr, 0);
    let cfg = function.cfg.as_ref().unwrap();
    for (bid, fat_loop) in fat_loops.fat_loops.iter() {
        let mut branchs = vec![];
        let mut unions: BTreeSet<BlockId> = BTreeSet::new();
        for natural_loop in natural_loops.iter() {
            let header = natural_loop.loop_header;
            let bodys = natural_loop.loop_body.clone();
            if header == *bid {
                unions.append(&mut bodys.clone());
            }
        }
        for union in unions.iter() {
            let children = cfg.successors(*union);
            for child in children {
                if !unions.contains(child) {
                    branchs.push(*union);
                }
            }
        }

        for branch in branchs.iter() {
            let content = cfg.content(*branch);
            let (mut lower, mut upper) = (0, 0);
            match content {
                BlockContent::Basic { lower: _lower, upper: _upper } => {
                    lower = *_lower;
                    upper = *_upper;
                },
                _ => { continue; }
            }
            let bc = &function.code[upper as usize];
            // println!("{:?}", bc);
            // println!("{:#?}",fat_loop.mut_targets.values());
            // println!("{:#?}", fat_loop.val_targets);
            match bc {
                Bytecode::Branch(_, then_label, else_label, src) => {
                    let cond = data_depent.get(*src);
                    // let mut res = "".to_string();
                    // cond.display(&mut res, stbgr);
                    // println!("{}", res);
                    println!("{} {}", src, cond.is_const());
                },
                _ => {
                    continue;
                }
            }
        }
    }

    let mut ret_flag = false;
    ret_flag
}