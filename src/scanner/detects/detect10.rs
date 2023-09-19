// RepeatedFunctionCall
use crate::{
    move_ir::{generate_bytecode::FunctionInfo, packages::Packages, utils},
    scanner::{detectors::AbstractDetector, result::*},
};
use move_model::{
    model::{FunId, ModuleId},
    symbol::SymbolPool,
    ty::Type,
};
use move_stackless_bytecode::{
    stackless_bytecode::{Bytecode, Operation},
    stackless_control_flow_graph::BlockContent,
};

pub struct Detector10<'a> {
    packages: &'a Packages<'a>,
    content: DetectContent,
}
use std::collections::{HashMap, HashSet};
impl<'a> AbstractDetector<'a> for Detector10<'a> {
    fn new(packages: &'a Packages<'a>) -> Self {
        Self {
            packages,
            content: DetectContent::new(Severity::Minor, DetectKind::RepeatedFunctionCall),
        }
    }
    fn run(&mut self) -> &DetectContent {
        for (mname, &ref stbgr) in self.packages.get_all_stbgr().iter() {
            self.content.result.insert(mname.to_string(), Vec::new());
            let symbol_pool = &stbgr.symbol_pool;
            for (idx, function) in stbgr.functions.iter().enumerate() {
                // 跳过 native 函数
                if utils::is_native(idx, stbgr) {
                    continue;
                }
                // 通过 funid 得到函数名，并构造最终输入：f1(f2,f3)，指函数 f1 中重复调用了 f2 和 f3
                if let Some(funids) = self.detect_repeated_function_call(function) {
                    let res = format!(
                        "{}({})",
                        function.name,
                        funids
                            .into_iter()
                            .map(|funid| { self.get_function_name(symbol_pool, &funid) })
                            .collect::<Vec<String>>()
                            .join(",")
                    );
                    self.content.result.get_mut(mname).unwrap().push(res);
                }
            }
        }
        &self.content
    }
}

impl<'a> Detector10<'a> {
    fn detect_repeated_function_call(&self, function: &FunctionInfo) -> Option<Vec<FunId>> {
        let cfg = function.cfg.as_ref().unwrap();
        let code = &function.code;
        // stack 用于迭代 dfs，其栈顶元素为下一个要访问的区块id（block_id）
        let mut stack = Vec::new();
        // path 作为stack 中压入的区块的历史记录
        let mut path = Vec::new();
        // func_map 中，key 为 block_id，value 为该代码块中调用的函数id（FunId）、参数、参数类型、函数所属模块
        let mut func_map = HashMap::new();
        // var_sets 记录等价的参数值，例如 var_sets=[($0,$1,$2),($3,$4)]，此时说明变量 $0,$1,$2 的值相同，$3,$4 的值相同。
        let mut var_sets = Vec::new();
        // 代码入口块
        let entry_block_id = cfg.entry_block();
        stack.push(&entry_block_id);
        path.push(&entry_block_id);
        let mut repeated_funids = Vec::new();
        // dfs 深度遍历
        while !stack.is_empty() {
            // 当前块以访问，从 stack 中弹出，注意，path 并不会弹出
            let block_id = stack.pop().unwrap();
            let block_content = cfg.content(*block_id);
            cfg.successors(*block_id)
                .iter()
                .for_each(|successor_block_id| {
                    // 若 path 中不存在后继块，则将后继块压入 stack 和 path 中
                    // 这是为了避免 while 循环：例如 path=[0,1]，此时 1 的后继块为 0，那就说明有循环，应该跳过循环
                    if !path.contains(&successor_block_id) {
                        stack.push(successor_block_id);
                        path.push(successor_block_id);
                    }
                });
            match *block_content {
                // 当前区块为代码块
                BlockContent::Basic { lower, upper } => {
                    let mut funcs = Vec::new();
                    for (_offset, bytecode) in
                        (lower..=upper).zip(&code[(lower as usize)..=(upper as usize)])
                    {
                        match bytecode {
                            Bytecode::Call(_, dsts, oper, args, _aa) => match oper {
                                // 获取当前代码块调用的所有函数id、参数、参数类型、所属模块，将其写入 func_map
                                Operation::Function(mid, funid, targs) => {
                                    funcs.push((mid, funid, args, targs));
                                }
                                // 如果是引用，则记录引用链。
                                // 例如 a=&b c=&b 则将 (a,b,c) 作为一个 var_set，存入 var_sets 中
                                // 因为通常调用函数时，不会出现 f(a) f(b) 的情况，而只会有 f(b) f(c) ，因此这么存问题不大
                                Operation::BorrowLoc
                                | Operation::BorrowGlobal(..)
                                | Operation::BorrowField(..)
                                | Operation::FreezeRef
                                // | Operation::WriteRef
                                | Operation::ReadRef => {
                                    // 当操作符为引用时，args 和 dsts 长度均为 1
                                    if args.len() == 1 && dsts.len() == 1 {
                                        let mut flag = false;
                                        var_sets.iter_mut().for_each(
                                            |var_set: &mut HashSet<&usize>| {
                                                if var_set.contains(&args[0]) {
                                                    var_set.insert(&dsts[0]);
                                                    flag = true
                                                }
                                            },
                                        );
                                        if flag {
                                            continue;
                                        }
                                        let mut var_set = HashSet::new();
                                        var_set.insert(&dsts[0]);
                                        var_set.insert(&args[0]);
                                        var_sets.push(var_set);
                                    } else {
                                        println!("error：args or dsts len!=1")
                                    }
                                }

                                _ => {}
                            },
                            // copy move sstore 均为赋值语句
                            Bytecode::Assign(_, dst, src, _askind) => {
                                let mut flag = false;
                                var_sets
                                    .iter_mut()
                                    .for_each(|var_set: &mut HashSet<&usize>| {
                                        // 假设 a=b，var_sets=[(b,c)]，此时说明 b=c，又因为 a=b，因此 a=b=c
                                        // 将其写入 var_sets 中，即[(a,b,c)]
                                        if var_set.contains(src) {
                                            var_set.insert(dst);
                                            flag = true
                                        }
                                    });
                                // 若没有找到，则新建一个set，假设 a=b var_sets=[(c,d)]
                                // 则新建：var_sets=[(a,b),(c,d)]
                                if flag {
                                    continue;
                                }
                                let mut var_set = HashSet::new();
                                var_set.insert(dst);
                                var_set.insert(src);
                                var_sets.push(var_set);
                            }
                            _ => {}
                        }
                    }
                    func_map.insert(block_id, funcs);
                    // println!("{:?}", block_content);
                }

                // 当前代码块为入口块或结束块。（其实写的是结束块的处理逻辑）
                BlockContent::Dummy => {
                    // 判断 func_map 中是否存在重复的函数id
                    // seen_funids 中存储已经出现过一次的 funid，及其对应的参数和参数类型
                    let mut seen_func: HashMap<FunId, (&ModuleId, &Vec<usize>, &Vec<Type>)> =
                        HashMap::new();

                    // 拿出所有的 FunId
                    for funcs in func_map.values() {
                        for (&ref mid, &funid, &ref args, &ref targs) in funcs {
                            // 已被记录重复调用，跳过
                            if repeated_funids.contains(&funid) {
                                continue;
                            }
                            // 首次出现，记录为已发现
                            if !seen_func.contains_key(&funid) {
                                seen_func.insert(funid, (mid, args, targs));
                                continue;
                            }
                            // funid 第二次重复出现，可能是重复调用
                            let mut find_diff = false;

                            let &(old_mid, old_args, old_targs) = seen_func.get(&funid).unwrap();

                            // 一、判断模块名称是否相同
                            // 二、判断参数类型是否相同，针对于泛型的方法。
                            if !mid.eq(old_mid) || !targs.eq(old_targs) {
                                find_diff = true;
                            }
                            // 三、判断所有参数是否相同
                            for (&old_arg, &arg) in old_args.iter().zip(args.iter()) {
                                if find_diff {
                                    break;
                                }
                                //TODO: 未考虑 f1(f2()) 的情况，即不先赋值，直接传入函数
                                for var_set in var_sets.iter() {
                                    // 每个 var_set 内部的值都认为是相同的
                                    if (var_set.contains(&old_arg) && !var_set.contains(&arg))
                                        || (!var_set.contains(&old_arg) && var_set.contains(&arg))
                                    {
                                        // 找到一个不同的参数
                                        find_diff = true;
                                        break;
                                    }
                                }
                            }
                            // 若没有发现参数不同；或参数类型不同的情况，则视为重复调用
                            if !find_diff {
                                repeated_funids.push(funid);
                            }
                        }
                    }

                    /*
                     *  设 cfg 为 0->1 1->2 1-3 2->4 3->4
                     *  该 cfg 存在两条路径 0->1->2->4 和 0->1->3->4
                     *  执行完 0->1->2->4 后：
                     *  stack=[3]
                     *  path =[0,1,3,2,4]
                     *  func_map={0:[...],1:[...],2:[...],4:[...]}
                     *  因此如果我们想分析执行分支 0->1->3->4 是否存在重复的函数调用，需要删除func_map中 2、4 的记录，保留 0、1 的记录
                     *  那么，先从 path 中弹出 4，再弹出 2，最后发现其栈顶和stack的栈顶元素相同，则不再进行删除操作。
                     *  如此，成功在 func_map 中删除了 2、4，而保留了0、1的记录
                     *  TIP: 在实际执行中，0 和 4 均为 dummpy 的空块
                     * */
                    if let Some(&stack_block_id) = stack.last() {
                        while let Some(&path_block_id) = path.last() {
                            if stack_block_id == path_block_id {
                                break;
                            }
                            path.pop();
                            func_map.remove(&path_block_id);
                        }
                    }
                }
            }
        }
        if repeated_funids.is_empty() {
            None
        } else {
            Some(repeated_funids)
        }
    }

    fn get_function_name(&self, symbol_pool: &SymbolPool, funid: &FunId) -> String {
        funid.symbol().display(symbol_pool).to_string()
    }

    // fn bfs(&self, function: &FunctionInfo) {
    //     let mut queue = VecDeque::new();
    //     let cfg = function.cfg.as_ref().unwrap();
    //     let entry_block_id = cfg.entry_block();
    //     queue.push_front(entry_block_id);
    //     let mut visited = Vec::new();
    //     while !queue.is_empty() {
    //         let block_id = queue.pop_back().unwrap();
    //         let block_content = cfg.content(block_id);
    //         cfg.successors(block_id).iter().for_each(
    //             |successor_block_id| {
    //                 // 如果后继节点未被访问过
    //                 if !visited.contains(successor_block_id){
    //                     visited.push(successor_block_id.clone());
    //                     queue.push_front(successor_block_id.clone());
    //                 }
    //             },
    //         );
    //         match block_content {
    //             BlockContent::Basic { lower, upper } => {
    //                 println!("{:?}", block_content);
    //             }
    //             BlockContent::Dummy => {
    //                 println!("{:?}", block_content);
    //             }
    //         }
    //     }
    // }
}
