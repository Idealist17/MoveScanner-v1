// // unused_private_functions

// use std::collections::{BTreeMap, BTreeSet};

// use move_binary_format::{file_format::{Bytecode as MoveBytecode, Visibility}, internals::ModuleIndex, views::FunctionDefinitionView, access::ModuleAccess};
// use move_model::{model::{QualifiedId, FunId, ModuleId}, pragmas::CONDITION_SUSPENDABLE_PROP};
// use crate::move_ir::generate_bytecode::StacklessBytecodeGenerator;
// use petgraph::{graph::{Graph, DiGraph, NodeIndex}, Direction, data::Build};
// use move_stackless_bytecode::stackless_bytecode::{
//     Bytecode, Operation
// };


// impl<'a> StacklessBytecodeGenerator<'a> {
//     pub fn build_call_graph(&mut self) {
//         let mut graph: Graph<QualifiedId<FunId>, ()> = DiGraph::new();
//         let mut nodes: BTreeMap<QualifiedId<FunId>, NodeIndex> = BTreeMap::new();
//         for func_id in self.module_data.function_idx_to_id.values() {
//             let qid = QualifiedId { 
//                 module_id: self.module_data.id, 
//                 id: *func_id,
//             };
//             let node_idx = graph.add_node(qid);
//             nodes.insert(qid, node_idx);
//         }

//         for (idx, func_id) in self.module_data.function_idx_to_id.iter() {
//             let function = &self.functions[idx.into_index()];
//             let qid = QualifiedId { 
//                 module_id: self.module_data.id, 
//                 id: *func_id,
//             };
//             let src_idx = nodes.get_mut(&qid).unwrap();
//             let called: BTreeSet<_> = function.code
//             .iter()
//             .filter_map(|c| {
//                 if let Bytecode::Call(_, _, Operation::Function(mid, fid, _), _, _) = c {
//                     Some(QualifiedId { 
//                         module_id: *mid, 
//                         id: *fid
//                     })
//                 } else {
//                     None
//                 }
//             }).collect();
            
//             for called_qid in called {
//                 let dst_idx = nodes.entry(called_qid).or_insert_with(||{
//                     graph.add_node(called_qid)
//                 });
//                 let dst_idx = dst_idx.;
//                 graph.add_edge(*src_idx, , ());
//             }
//         }
//         self.call_graph = graph;
//         self.func_to_node = nodes;
//     }
// }

// fn get_unused_functions<'a>(stbgr: &'a StacklessBytecodeGenerator) -> Vec<&'a QualifiedId<FunId>> {
//     let mut unused_functions: Vec<&QualifiedId<FunId>> = vec![];
//     for (fid, nid) in stbgr.func_to_node.iter() {
//         // 调用边，即入边
//         let neighbors = stbgr.call_graph.neighbors_directed(*nid, Direction::Incoming);
//         if neighbors.into_iter().next().is_none() {
//             unused_functions.push(fid);
//         }
//     }
//     unused_functions
// }

// pub fn detect_unused_private_functions(stbgr: &StacklessBytecodeGenerator) -> Vec<FunId> {
//     let mut unused_private_functions: Vec<FunId> = vec![];
//     let unused_functions = get_unused_functions(stbgr);
//     for func in unused_functions {
//         let function_data = stbgr.module_data.function_data.get(&func.id).unwrap();
//         let view = FunctionDefinitionView::new(
//             stbgr.module, 
//             stbgr.module.function_def_at(function_data.def_idx)
//         );
//         if view.visibility() == Visibility::Private && !view.is_entry() 
//             && !view.name().as_str().starts_with("init"){
//                 unused_private_functions.push(func.id);
//         }
//     }
//     unused_private_functions
// }