use crate::cli::parser::{Args, IR};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// 终端输出格式
#[derive(Debug, Clone)]
pub enum TerminalFormat {
    Block, // default
    None,
}
#[derive(Debug, Clone)]
pub struct Options {
    // 源码目录可能不存在
    pub sources_path: Option<PathBuf>,
    pub bytecode_path: PathBuf,
    pub output_path: PathBuf,
    pub terminal_format: TerminalFormat,
    pub ir_type: Option<IR>,
}

impl Options {
    // TODO: 增加从配置文件读入配置
    pub fn build_options(args: Args) -> Self {
        let path = Path::new(&args.path);
        // sources 文件夹必须是当前给定目录下的直接子目录
        let mut sources_path = None;
        let mut build_path = None;
        if let Ok(entries) = fs::read_dir(Path::new(path)) {
            for entry in entries.into_iter().filter_map(|e| e.ok()) {
                if entry.path().is_dir() {
                    if entry.file_name() == "sources" {
                        sources_path = Some(entry.path());
                        // println!("sources： {}", entry.path().display());
                    } else if entry.file_name() == "build" {
                        build_path = Some(entry.path());
                        // println!("build： {}", entry.path().display());
                    }
                }
            }
        }
        // 如果存在 build 路径，则递归查找 bytecode_modules
        let mut bytecode_path = PathBuf::new();
        if let Some(build_path) = build_path {
            for entry in WalkDir::new(build_path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_dir() {
                    if entry.file_name() == "bytecode_modules" {
                        bytecode_path = entry.path().to_path_buf();
                        // println!("bytecode_modules:  {}", entry.path().display());
                    }
                }
            }
        } else {
            // 若不存在 build 路径，但存在 sources 路径，则打出提醒，说不定是使用者忘记编译了
            if sources_path.is_some() {
                println!("Detected /sources, but not found /build. Please check if the project has been compiled.");
            }
            // 若不存在 build 路径，则将传入的路径视作 bytecode_path
            bytecode_path = path.to_path_buf();

            // 注意：此时即便有 sources 路径，也视为无效
            sources_path = None;
        }

        let terminal_format: TerminalFormat;
        if args.none {
            terminal_format = TerminalFormat::None;
        } else {
            terminal_format = TerminalFormat::Block;
        }
        Self {
            sources_path: sources_path,
            bytecode_path: bytecode_path,
            output_path: PathBuf::from(args.output.unwrap()),
            terminal_format: terminal_format,
            ir_type: args.ir_type,
        }
    }
}
