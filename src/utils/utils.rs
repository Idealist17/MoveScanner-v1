// 
use std::path::PathBuf;
use std::fs;

use move_model::model::ModuleEnv;


// 依赖的 module 的 address
const DEPADDRESSES: [&str; 2] = ["0x1", "0x3"];


// get all .mv files in dir and subdir
pub fn visit_dirs(dir: &PathBuf, paths: &mut Vec<PathBuf>) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, paths);
            } else {
                paths.push(path);
            }
        }
    }
}

pub fn is_dep_module(module_env: &ModuleEnv) -> bool {
    let mut is_dep = false;
    let module_addr = module_env.get_full_name_str();
    for addr in DEPADDRESSES {
        if module_addr.starts_with(addr) {
            is_dep = true;
            break;
        }
    }
    return is_dep
}

use anyhow::anyhow;
use move_stackless_bytecode::{
    function_target_pipeline::FunctionTargetPipeline,
    usage_analysis::UsageProcessor,
};
// IR 优化
pub fn get_tested_transformation_pipeline(
    dir_name: &str,
) -> anyhow::Result<Option<FunctionTargetPipeline>> {
    match dir_name {
        "from_move" => Ok(None),
        "usage_analysis" => {
            let mut pipeline = FunctionTargetPipeline::default();
            pipeline.add_processor(UsageProcessor::new());
            Ok(Some(pipeline))
        }
        _ => Err(anyhow!(
            "the sub-directory `{}` has no associated pipeline to test",
            dir_name
        )),
    }
}