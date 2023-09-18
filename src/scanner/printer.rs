use crate::{
    cli::parser::IR,
    move_ir::{
        control_flow_graph::generate_cfg_in_dot_format,
        packages::{build_compiled_modules, Packages},
    },
    scanner::option::Options,
};
use move_binary_format::access::ModuleAccess;
use petgraph::dot::Dot;
use std::{fs, io::Write, path::PathBuf};
pub struct Printer {
    options: Options,
    // pub result: Result,
}

impl Printer {
    pub fn new(options: Options) -> Self {
        Self { options }
    }

    pub fn run(&mut self) {
        let cms = build_compiled_modules(&self.options.bytecode_path);
        let packages = Packages::new(&cms);
        let printer_path = PathBuf::from("./printer");
        if !printer_path.exists() {
            // 如果父目录不存在，创建它
            fs::create_dir_all(&printer_path).expect("create path failed.");
        }
        // }else{
        //     fs::remove_dir_all(&printer_path).expect("remove dir failed");
        //     fs::create_dir_all(&printer_path).expect("create path failed.");
        // }
        // 遍历packages中的stbgr
        for (mname, &ref stbgr) in packages.get_all_stbgr().iter() {
            match self.options.ir_type {
                Some(IR::CFG) => {
                    for (idx, function) in stbgr.functions.iter().enumerate() {
                        let cfg_path =
                            printer_path.join(format!("cfg/{}/{}.dot", mname, function.name));
                        generate_cfg_in_dot_format(&stbgr.functions[idx], cfg_path, &stbgr);
                        // function.cfg.as_ref().unwrap().display();
                    }
                    println!("cfg has been generated to folder ./printer/cfg success");
                }
                Some(IR::CG) => {
                    let graph = stbgr.call_graph2str();
                    let dot_graph = format!(
                        "{}",
                        Dot::with_attr_getters(&graph, &[], &|_, _| "".to_string(), &|_, _| {
                            "shape=box".to_string()
                        })
                    );
                    let cg_path = printer_path.join(format!("cg/{}.dot", mname));
                    if let Some(parent) = cg_path.parent() {
                        if !parent.exists() {
                            // 如果父目录不存在，创建它
                            fs::create_dir_all(parent).expect("create path failed.");
                        }
                    }
                    match fs::write(&cg_path, &dot_graph) {
                        Ok(_) => {
                            println!("call graph has been generated to folder ./printer/cg success",)
                        }
                        Err(e) => eprintln!("Error writing file: {}", e),
                    }
                }
                Some(IR::SB) => {
                    let sb_path = printer_path.join(format!("sb/{}.sb", mname));
                    if let Some(parent) = sb_path.parent() {
                        if !parent.exists() {
                            // 如果父目录不存在，创建它
                            fs::create_dir_all(parent).expect("create path failed.");
                        }
                    }
                    let mut sb_file = fs::File::create(&sb_path).expect("无法创建文件");
                    writeln!(&mut sb_file, "{}", stbgr.display(true, None)).expect("写入文件失败");
                    // println!("{}", stbgr.display(true, None));
                    println!(
                        "stackless bytecode has been generated to file {} success",
                        sb_path.to_string_lossy()
                    );
                }
                Some(IR::CM) => {
                    let cm_path = printer_path.join(format!("cm/{}.cm", mname));
                    if let Some(parent) = cm_path.parent() {
                        if !parent.exists() {
                            // 如果父目录不存在，创建它
                            fs::create_dir_all(parent).expect("create path failed.");
                        }
                    }
                    let mut cm_file = fs::File::create(&cm_path).expect("无法创建文件");
                    writeln!(&mut cm_file, "{:#?}", stbgr.module).expect("写入文件失败");
                    // println!("{:#?}", stbgr.module);
                    println!(
                        "compile module has been generated to file {} success",
                        cm_path.to_string_lossy()
                    );
                }
                Some(IR::FS) => {
                    stbgr.print_func_signature();
                }
                Some(IR::DU) => {
                    for (_idx, function) in stbgr.functions.iter().enumerate() {
                        println!("{:?}", &function.def_attrid);
                        println!("{:?}", &function.use_attrid);
                    }
                }
                _ => {}
            }
        }
    }
}
