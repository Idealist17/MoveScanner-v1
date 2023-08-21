use crate::{
    move_ir::{
        packages::{build_compiled_modules, Packages},
        utils,
    },
    scanner::{
        detects::{
            detect1::Detector1, detect2::Detector2, detect3::Detector3, detect4::Detector4,
            detect5::Detector5, detect6::Detector6, detect7::Detector7, detect8::Detector8,
        },
        option::{Options, TerminalFormat},
        result::*,
    },
    utils::utils::print_logo,
};
use num::ToPrimitive;
use regex::Regex;
use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Write},
    time::Instant,
};
use walkdir::WalkDir;
pub trait AbstractDetector<'a> {
    fn new(packages: &'a Packages<'a>) -> Self
    where
        Self: Sized;
    fn run(&mut self) -> &DetectContent;
}

pub struct Detectors {
    pub options: Options,
    pub result: Result,
}

impl Detectors {
    pub fn new(options: Options) -> Self {
        Self {
            options,
            result: Result::empty(),
        }
    }

    pub fn run(&mut self) {
        let clock = Instant::now();
        // build package
        let cms = build_compiled_modules(&self.options.bytecode_path);
        let packages = Packages::new(&cms);
        self.init_result(&packages);
        // register detector
        let mut detectors: Vec<Box<dyn AbstractDetector>> = vec![
            Box::new(Detector1::new(&packages)),
            Box::new(Detector2::new(&packages)),
            Box::new(Detector3::new(&packages)),
            Box::new(Detector4::new(&packages)),
            Box::new(Detector5::new(&packages)),
            Box::new(Detector6::new(&packages)),
            Box::new(Detector7::new(&packages)),
            Box::new(Detector8::new(&packages)),
        ];
        // run detectors
        for detector in detectors.iter_mut() {
            let detect_content = detector.run();
            self.merge_result(detect_content);
        }
        self.complete_result(clock);
    }

    pub fn output_result(&self) {
        let json_result = serde_json::to_string(&self.result).ok().unwrap();
        // 输出到指定目录
        if let Some(dir_path) = self.options.output_path.parent() {
            // 不存在则递归创建路径目录
            if !dir_path.exists() {
                if let Err(error) = fs::create_dir_all(dir_path) {
                    println!("Error creating directories: {:?}", error);
                }
            }
        }
        let mut file =
            fs::File::create(self.options.output_path.clone()).expect("Failed to create json file");
        file.write(json_result.as_bytes())
            .expect("Failed to write to json file");

        match self.options.terminal_format.clone() {
            TerminalFormat::Block => {
                print_logo();
                println!("{}", self.result);
            }
            _ => {}
        }
    }

    /// 为每个 module 初始化 ModuleInfo，用于记录对应 module 的检测结果：
    /// 1. constant_count： 常量数量
    /// 2. function_count： 函数数量（区分 native）
    /// 3. location： 若输入路径为项目路径（包含 sources），则给出 module 的源码位置
    fn init_result(&mut self, packages: &Packages) {
        let locations = self.find_module_path(&packages.get_module_names());

        for (module_name, &ref stbgr) in packages.get_all_stbgr().iter() {
            let mut module_info = ModuleInfo::empty();
            module_info.constant_count = stbgr.module.constant_pool.len();
            *module_info
                .function_count
                .get_mut(&FunctionType::All)
                .unwrap() = stbgr.functions.len();
            module_info.location = locations.get(module_name).unwrap().clone();
            for (idx, _function) in stbgr.functions.iter().enumerate() {
                if utils::is_native(idx, stbgr) {
                    *module_info
                        .function_count
                        .get_mut(&FunctionType::Native)
                        .unwrap() += 1;
                }
            }
            self.result.add_module(module_name.to_string(), module_info);
        }
    }

    // 将每个 detector 检测结果同步到 result 中
    fn merge_result(&mut self, detect_content: &DetectContent) {
        let kind = detect_content.kind.clone();
        for (module_name, detect_res) in detect_content.result.iter() {
            detect_res.iter().for_each(|r| {
                self.result
                    .modules
                    .get_mut(module_name)
                    .unwrap()
                    .detectors
                    .get_mut(&kind)
                    .unwrap()
                    .push(r.to_string());
            })
        }
    }

    /// 收尾工作:
    /// 1. 总耗时
    /// 2. 若 module 未检出任何漏洞，则标记为 pass，否则 wrong
    /// 3. result 汇总 module 状态信息
    fn complete_result(&mut self, clock: Instant) {
        self.result.total_time = clock.elapsed().as_micros().to_usize().unwrap();
        // let module_count = self.result.modules.len();
        for (module_name, module_info) in self.result.modules.iter_mut() {
            let mut pass = true;
            for (_detector_type, values) in module_info.detectors.iter() {
                if !values.is_empty() {
                    pass = false;
                }
            }
            if pass {
                self.result
                    .modules_status
                    .get_mut(&Status::Pass)
                    .unwrap()
                    .push(module_name.to_string());
                module_info.status = Status::Pass;
            } else {
                self.result
                    .modules_status
                    .get_mut(&Status::Wrong)
                    .unwrap()
                    .push(module_name.to_string());
                module_info.status = Status::Wrong;
            }
        }
    }
    fn find_module_path(
        &self,
        module_name_list: &Vec<String>,
    ) -> HashMap<ModuleName, Option<Location>> {
        let mut res = HashMap::new();
        let mut used_sources_path = Vec::new();
        let mut all_sources_path = Vec::new();
        if let Some(source_path) = self.options.sources_path.clone() {
            for entry in WalkDir::new(source_path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file()
                    && entry.file_name().to_str().unwrap().ends_with(".move")
                {
                    all_sources_path.push(entry.path().to_path_buf());
                }
            }
        } else {
            // 不存在 sources path 的时候，直接返回空
            for module_name in module_name_list {
                res.insert(module_name.to_string(), None);
            }
            return res;
        }
        for module_name in module_name_list {
            let re =
                Regex::new(format!("module .*::{}", module_name).to_string().as_str()).unwrap();
            let mut find = false;
            // 1. 首先查询未使用过的源码
            for source_path in all_sources_path.iter() {
                if !used_sources_path.contains(&source_path) {
                    let file = fs::File::open(source_path).unwrap();
                    let reader = BufReader::new(file);
                    for (line_num, line) in reader.lines().enumerate() {
                        if let Ok(line) = line {
                            if re.is_match(&line) {
                                // println!(
                                //     "Found '{}' in file '{}' at line {}",
                                //     module_name,
                                //     source_path.display(),
                                //     line_num + 1
                                // );
                                let location =
                                    format!("{}:{}", source_path.display(), line_num + 1);
                                res.insert(module_name.to_string(), Some(location));
                                used_sources_path.push(source_path);
                                find = true;
                                break;
                            }
                        }
                    }
                }

                if find {
                    break;
                }
            }
            // 若在未查询过的源码中没找到对应 module 的定义，那么在已经查询到过 module 定义的源码中查找（该文件定义了多个 module）
            if !find {
                for used_source_path in used_sources_path.iter() {
                    let file = fs::File::open(used_source_path).unwrap();
                    let reader = BufReader::new(file);
                    for (line_num, line) in reader.lines().enumerate() {
                        if let Ok(line) = line {
                            if re.is_match(&line) {
                                // println!(
                                //     "Found '{}' in file '{}' at line {}",
                                //     module_name,
                                //     used_source_path.display(),
                                //     line_num + 1
                                // );
                                let location =
                                    format!("{}:{}", used_source_path.display(), line_num + 1);
                                res.insert(module_name.to_string(), Some(location));
                                find = true;
                                break;
                            }
                        }
                    }
                }
            }
            // 若还是没找到，报错
            if !find {
                res.insert(module_name.to_string(), None);
                println!("Info: {} not found in source code！", module_name);
            }
        }
        res
    }
}
