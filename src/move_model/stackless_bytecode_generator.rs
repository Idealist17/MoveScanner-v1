use move_binary_format::file_format::Visibility;
use move_binary_format::CompiledModule;
use move_bytecode_utils::Modules;
use move_model::run_bytecode_model_builder;
use move_model::symbol::Symbol;
use std::fmt::Write;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::vec;
use std::fs;
use crate::utils::utils;

use codespan_reporting::{diagnostic::Severity, term::termcolor::Buffer};
use move_model::{ast::ModuleName, model::GlobalEnv, ty::Type};
use move_package::{source_package::layout::SourcePackageLayout, BuildConfig, ModelConfig};
use move_stackless_bytecode::{
    function_target::FunctionTarget,
    function_target_pipeline::{FunctionTargetsHolder, FunctionVariant},
    stackless_bytecode::Bytecode,
    stackless_bytecode_generator::StacklessBytecodeGenerator,
    stackless_control_flow_graph::{generate_cfg_in_dot_format, StacklessControlFlowGraph},
};

const SUIDEPENDENCYDIR: &str = "./testdata/dependencies/sui/";
const APTOSDEPENDENCYDIR: &str = "./testdata/dependencies/aptos/";

pub fn print_targets_for_test(
    env: &GlobalEnv,
    header: &str,
    targets: &FunctionTargetsHolder,
) -> String {
    let mut text = String::new();
    writeln!(&mut text, "============ {} ================", header).unwrap();
    for module_env in env.get_modules() {
        if utils::is_dep_module(&module_env) {
            continue;
        }
        for func_env in module_env.get_functions() {
            for (variant, target) in targets.get_targets(&func_env) {
                if !target.data.code.is_empty() || target.func_env.is_native_or_intrinsic() {
                    target.register_annotation_formatters_for_test();
                    writeln!(&mut text, "\n[variant {}]\n{}", variant, target).unwrap();
                }
            }
        }
    }
    text
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Symbol,
    pub module_name: ModuleName,
    pub visibility: Visibility,
    pub is_entry: bool,
    pub params: Vec<Type>,
    pub rets: Vec<Type>,
    pub bytecodes: Vec<Bytecode>,
}

pub fn reroot_path(path: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    let path: PathBuf = path.unwrap_or_else(|| PathBuf::from("."));
    // 定位包的根目录 即 Move.toml
    let rooted_path = SourcePackageLayout::try_find_root(&path.canonicalize()?)?;
    std::env::set_current_dir(&rooted_path).unwrap();
    Ok(PathBuf::from("."))
}

pub fn source2stackless_ir(path: &str, pipe_list: &str) -> (GlobalEnv, FunctionTargetsHolder) {
    let path = Path::new(path);
    let config = BuildConfig {
        // 先使用默认配置
        ..Default::default()
    };
    let env = config
        .move_model_for_package(
            &reroot_path(Option::Some(path.to_path_buf())).unwrap(),
            ModelConfig {
                // 不分析依赖 不屏蔽任何文件
                all_files_as_targets: false,
                target_filter: None,
            },
        )
        .expect("Failed to create GlobalEnv!");

    let mut targets = FunctionTargetsHolder::default();
    if env.has_errors() {
        let mut error_writer = Buffer::no_color();
        env.report_diag(&mut error_writer, Severity::Error);
        println!(
            "{}",
            String::from_utf8_lossy(&error_writer.into_inner()).to_string()
        );
    } else {
        for module_env in env.get_modules() {
            for func_env in module_env.get_functions() {
                targets.add_target(&func_env);
            }
        }

        // text += &print_targets_for_test(&env, "initial translation from Move", &targets);

        // 做分析 参照 pipeline 写分析代码
        let pipeline_opt = utils::get_tested_transformation_pipeline(pipe_list).unwrap();
        // Run pipeline if any
        if let Some(pipeline) = pipeline_opt {
            pipeline.run(&env, &mut targets);
            // let processor = pipeline.last_processor();
        };

        // add Warning and Error diagnostics to output
        let mut error_writer = Buffer::no_color();
        if env.has_errors() || env.has_warnings() {
            env.report_diag(&mut error_writer, Severity::Warning);
            println!("{}", &String::from_utf8_lossy(&error_writer.into_inner()));
        }
    };
    (env, targets)
}

pub fn get_from_bytecode_modules(dir: &str) -> (GlobalEnv, FunctionTargetsHolder) {
    let mut all_modules: Vec<CompiledModule> = Vec::new();
    // 需要分析的 mv
    let dir = PathBuf::from(dir);
    let mut paths = Vec::new();
    utils::visit_dirs(&dir, &mut paths);
    // 常用的外部依赖，目前为止，默认全部加在进去，后续再做优化
    let dep_dir = PathBuf::from(APTOSDEPENDENCYDIR);
    utils::visit_dirs(&dep_dir, &mut paths);

    for filename in paths {
        let f = fs::File::open(filename).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();

        let cm = CompiledModule::deserialize(&buffer).unwrap();
        all_modules.push(cm);
    }

    let all_modules = Modules::new(&all_modules);
    let dep_graph = all_modules.compute_dependency_graph();
    let modules = dep_graph.compute_topological_order().unwrap();

    let env = run_bytecode_model_builder(modules).unwrap();
    let mut targets = FunctionTargetsHolder::default();
    if env.has_errors() {
        let mut error_writer = Buffer::no_color();
        env.report_diag(&mut error_writer, Severity::Error);
        println!(
            "{}",
            String::from_utf8_lossy(&error_writer.into_inner()).to_string()
        );
    } else {
        for module_env in env.get_modules() {
            for func_env in module_env.get_functions() {
                targets.add_target(&func_env);
            }
        }

        // text += &print_targets_for_test(&env, "initial translation from Move", &targets);

        // 做分析 参照 pipeline 写分析代码
        let pipeline_opt = utils::get_tested_transformation_pipeline("from_move").unwrap();
        // Run pipeline if any
        if let Some(pipeline) = pipeline_opt {
            pipeline.run(&env, &mut targets);
            // let processor = pipeline.last_processor();
        };

        // add Warning and Error diagnostics to output
        let mut error_writer = Buffer::no_color();
        if env.has_errors() || env.has_warnings() {
            env.report_diag(&mut error_writer, Severity::Warning);
            println!("{}", &String::from_utf8_lossy(&error_writer.into_inner()));
        }
    };
    (env, targets)
}

pub fn get_functions_from_globalenv(env: &GlobalEnv, targets: &FunctionTargetsHolder) -> Vec<Function> {
    let mut funtcionts = vec![];
    for module_env in env.get_modules() {
        if utils::is_dep_module(&module_env) {
            continue;
        }
        for func_env in module_env.get_functions() {
            let target = targets.get_target(&func_env, &FunctionVariant::Baseline);

            // func_env.is_native_or_intrinsic() ...
            let is_entry = func_env.is_entry();
            let visibility = func_env.visibility();
            let module_name = func_env.module_env.get_name().clone();
            let func_name = func_env.get_name();
            // 范型参数数量
            let tparams_count_all = func_env.get_type_parameter_count();
            let tparams_count_defined = func_env.get_type_parameter_count();
            // 参数及类型
            let mut params: Vec<Type> = vec![];
            let params_count = func_env.get_parameter_count();
            for idx in 0..params_count {
                let ty = func_env.get_local_type(idx);
                params.push(ty);
                let local_name = if target.has_local_user_name(idx) {
                    Some(target.get_local_name(idx))
                } else {
                    None
                };
            }
            // 返回值类型
            let mut rets = vec![];
            let return_count = func_env.get_return_count();
            for idx in 0..return_count {
                let return_type = target.get_return_type(idx).clone();
                rets.push(return_type);
            }
            // 所有左值类型
            let local_count = func_env.get_local_count();
            for idx in params_count..local_count {
                let ty = func_env.get_local_type(idx);
                let local_name = if target.has_local_user_name(idx) {
                    Some(target.get_local_name(idx))
                } else {
                    None
                };
            }

            let bytecodes = target.get_bytecode();
            // let label_offsets = Bytecode::label_offsets(bytecodes);
            // for (offset, code) in bytecodes.iter().enumerate() {
            //     println!(
            //         "{}",
            //         format!("{:>3}: {}", offset, code.display(&target, &label_offsets))
            //     );
            // }
            let function = Function {
                name: func_name,
                module_name,
                visibility,
                is_entry,
                params,
                rets,
                bytecodes: bytecodes.to_vec(),
            };
            funtcionts.push(function);
        }
    }
    funtcionts
}

pub fn get_cfg(env: &GlobalEnv) {
    for module_env in env.get_modules() {
        if utils::is_dep_module(&module_env) {
            continue;
        }
        for func_env in module_env.get_functions() {
            let generator = StacklessBytecodeGenerator::new(&func_env);
            let data = generator.generate_function();
            let func_target = FunctionTarget::new(&func_env, &data);
            // 到这里为止 只要能拿到 bytecode 即可继续进行分析
            let code = func_target.get_bytecode();
            let cfg = StacklessControlFlowGraph::new_forward(code);
            // CFG.dot
            let dot_graph = generate_cfg_in_dot_format(&func_target);
            std::fs::write(&"cfg.dot", &dot_graph).expect("generating dot file for CFG");
        }
    }
}
