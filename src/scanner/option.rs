use crate::cli::parser::{Args, IR};
use crate::scanner::compile::compile;
use crate::utils::utils::{find_path_by_dir_name, toml_file_count};
use std::path::PathBuf;
// use toml::Value;
// 终端输出格式
#[derive(Debug, Clone)]
pub enum TerminalFormat {
    Block, // default
    None,
}
#[derive(Debug, Clone)]
pub struct Options {
    // options from args
    pub sources_path: Option<PathBuf>,
    pub bytecode_path: PathBuf,
    pub output_path: PathBuf,
    pub terminal_format: TerminalFormat,
    pub ir_type: Option<IR>,
    // pub config:Value
}

impl Options {
    // TODO: 增加从配置文件读入配置
    pub fn build_options(args: Args) -> Self {
        let path = PathBuf::from(&args.path);
        // 只允许一个子项目
        assert!(toml_file_count(&path) < 2, "up to one project is allowed");
        let sources_path = find_path_by_dir_name(&path, "sources");
        let mut bytecode_path = Some(path.clone());
        // 若存在 sources 路径，则编译项目，再找 bytecode 路径
        // println!("Detected /sources, Try to compile.");
        if let Some(sources_path) = sources_path.clone() {
            assert!(compile(&sources_path), "compile project failed");
            bytecode_path = find_path_by_dir_name(&path, "bytecode_modules");
        }
        let terminal_format: TerminalFormat;
        if args.none {
            terminal_format = TerminalFormat::None;
        } else {
            terminal_format = TerminalFormat::Block;
        }
        Self {
            sources_path: sources_path,
            bytecode_path: bytecode_path.unwrap(),
            output_path: PathBuf::from(args.output.unwrap()),
            terminal_format: terminal_format,
            ir_type: args.ir_type,
        }
    }
}
