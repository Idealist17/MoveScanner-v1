// infinite_loop

use std::collections::BTreeSet;

use move_stackless_bytecode::{stackless_control_flow_graph::BlockContent, stackless_bytecode::Bytecode};

use crate::move_ir::{generate_bytecode::StacklessBytecodeGenerator, fatloop::get_loops, data_dependency::data_dependency, control_flow_graph::BlockId};


pub fn detect_infinite_loop(stbgr: &StacklessBytecodeGenerator, idx: usize) -> bool {
    let function = &stbgr.functions[idx];
    let (natural_loops, fat_loops) = get_loops(function);
    let data_depent = data_dependency(stbgr, idx);
    let cfg = function.cfg.as_ref().unwrap();
    let mut ret_flag = if fat_loops.fat_loops.len() > 0 {true} else {false};
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
                Bytecode::Branch(_, _, _, src) => {
                    let cond = data_depent.get(*src);
                    // let mut res = "".to_string();
                    // cond.display(&mut res, stbgr);
                    // println!("{}", res);
                    let mut conditions = vec![];
                    cond.loop_condition_from_copy(&mut conditions);
                    if conditions.len() == 0 {
                        continue;
                    }
                    for condition in conditions {
                        for block_id in unions.iter() {
                            let content = cfg.content(*block_id);
                            match content {
                                BlockContent::Basic { lower, upper } => {
                                    for offset in *lower..*upper {
                                        match function.code[offset as usize] {
                                            Bytecode::Assign(_, dst, _, _) => {
                                                if dst == condition {
                                                    ret_flag = false;
                                                }
                                            },
                                            _ => {}
                                        }
                                    }
                                },
                                _ => {}
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }
    ret_flag
}