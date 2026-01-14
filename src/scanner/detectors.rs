use crate::{
    move_ir::{
        packages::{build_compiled_modules, Packages},
        utils,
    },
    scanner::{
        option::{Options},
        result::*,
    },

};
use move_binary_format::access::ModuleAccess;
use num::ToPrimitive;
use regex::Regex;
use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Write},
    time::Instant,
};
use walkdir::WalkDir;

pub struct GraphGenerator {
    pub options: Options,
    pub result: Result,
}

impl GraphGenerator {
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

        // No detectors to run
        
        self.complete_result(clock);

        // Export Knowledge Graph
        let graph_output = crate::scanner::exporter::GraphExporter::export(&packages, &self.result);
        let graph_json = serde_json::to_string_pretty(&graph_output).expect("Failed to serialize graph");
        
        let mut graph_path = self.options.output_path.clone();
        if let Some(file_name) = graph_path.file_stem() {
            let mut new_name = file_name.to_os_string();
            // Just output_graph.json as expected by python script
            // The python script expects `output_graph.json` if we passed `output.json`.
            // The original logic appended `_graph.json`.
            new_name.push("_graph.json");
            graph_path.set_file_name(new_name);
        } else {
             graph_path.set_extension("graph.json");
        }

        if let Some(dir_path) = graph_path.parent() {
            if !dir_path.exists() {
                 let _ = fs::create_dir_all(dir_path);
            }
        }
        
        let mut file = fs::File::create(graph_path).expect("Failed to create graph json file");
        file.write_all(graph_json.as_bytes()).expect("Failed to write graph json");
    }

    /// Initialize ModuleInfo for each module
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
            
            // Extract Structs
            for (i, def) in stbgr.module.struct_defs().iter().enumerate() {
                 let def_idx = move_binary_format::file_format::StructDefinitionIndex(i as u16);
                 let handle = stbgr.module.struct_handle_at(def.struct_handle);
                 let name = stbgr.module.identifier_at(handle.name).to_string();
                 let abilities = utils::get_struct_abilities_strs(stbgr.module, def_idx);
                 module_info.structs.push(StructResult{
                     name,
                     abilities,
                     source_code: String::new(),
                 })
            }

            self.result.add_module(module_name.to_string(), module_info);
        }
    }

    fn complete_result(&mut self, clock: Instant) {
        self.result.total_time = clock.elapsed().as_micros().to_usize().unwrap();
        // Always pass since we aren't checking anything
        for (_module_name, module_info) in self.result.modules.iter_mut() {
            module_info.status = Status::Pass;
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
            for module_name in module_name_list {
                res.insert(module_name.to_string(), None);
            }
            return res;
        }
        for module_name in module_name_list {
            let re =
                Regex::new(format!(r"module .*::{}([{{\s]|$)", module_name).to_string().as_str()).unwrap();
            let mut find = false;
            // 1. Check unused sources
            for source_path in all_sources_path.iter() {
                if !used_sources_path.contains(&source_path) {
                    let file = fs::File::open(source_path).unwrap();
                    let reader = BufReader::new(file);
                    for (line_num, line) in reader.lines().enumerate() {
                        if let Ok(line) = line {
                            if re.is_match(&line) {
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
            // 2. Check used sources
            if !find {
                for used_source_path in used_sources_path.iter() {
                    let file = fs::File::open(used_source_path).unwrap();
                    let reader = BufReader::new(file);
                    for (line_num, line) in reader.lines().enumerate() {
                        if let Ok(line) = line {
                            if re.is_match(&line) {
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
            // 3. Not found
            if !find {
                res.insert(module_name.to_string(), None);
                println!("Info: {} not found in source code！", module_name);
            }
        }
        res
    }
}
