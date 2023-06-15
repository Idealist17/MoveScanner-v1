// // unused_private_functions

// fn get_unused_functions(ms: &MoveScanner) -> Vec<&QualifiedId<FunId>> {
//     let mut unused_funs: Vec<&QualifiedId<FunId>> = vec![];
//     for (fid, nid) in ms.fun_map.iter() {
//         if is_dep_module(&(ms.env).get_module(fid.module_id)) {
//             continue;
//         }
//         // 调用边，即入边
//         let neighbors = ms.call_graph.neighbors_directed(*nid, Direction::Incoming);
//         if neighbors.into_iter().next().is_none() {
//             unused_funs.push(fid);
//         }
//     }
//     unused_funs
// }

// pub fn detect_unused_private_functions(ms: &MoveScanner) -> Vec<&QualifiedId<FunId>> {
//     let mut unused_private_functions: Vec<&QualifiedId<FunId>> = vec![];
//     let unused_funs = get_unused_functions(ms);
//     for fun in unused_funs {
//         let fun_env = ms.env.get_function(*fun);
//         let visibility = fun_env.visibility();
//         match visibility {
//             Visibility::Private => unused_private_functions.push(fun),
//             _ => {},
//         }
//     }
//     unused_private_functions
// }