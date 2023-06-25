use std::{path::PathBuf, str::FromStr, fmt::format, fs};

use clap::Parser;
use MoveScanner::{
    cli::parser::*, 
    move_ir::{sbir_generator::{MoveScanner as Mc, Blockchain}, generate_bytecode::StacklessBytecodeGenerator, bytecode_display::display, control_flow_graph::generate_cfg_in_dot_format}, 
    utils::utils::{compile_module, self}, detect::{detect4::detect_infinite_loop, detect5::detect_unused_constants, detect6::detect_unused_private_functions}
};
use MoveScanner::{
    detect::{
        detect1::detect_unchecked_return,
        detect2::detect_overflow,
        detect3::detect_precision_loss,
        
        detect7::detect_unnecessary_type_conversion, 
        detect8::detect_unnecessary_bool_judgment, 
        }
};
use move_binary_format::access::ModuleAccess;

fn main() {
    let cli = Cli::parse();
    let dir = PathBuf::from(&cli.filedir);
    let mut paths = Vec::new();
    utils::visit_dirs(&dir, &mut paths, false);
    for filename in paths {
        let cm = compile_module(filename);
        let mut stbgr = StacklessBytecodeGenerator::new(&cm);
        stbgr.generate_function();
        stbgr.get_control_flow_graph();
        match &cli.command {
            Some(Commands::Printer { printer }) => {
                match &printer {
                    Some(Infos::CFG) => {
                        let dot_dir = "./dots";
                        if !fs::metadata(dot_dir).is_ok() {
                            match fs::create_dir(dot_dir) {
                                Ok(_) => {},
                                Err(err) => println!("Failed to create folder: {}", err)
                            };
                        }
                        for (idx, function) in stbgr.functions.iter().enumerate() {
                            let name = cm.identifier_at(cm.function_handle_at(cm.function_defs[idx].function).name);
                            let filename = PathBuf::from(format!("{}/{}.dot", dot_dir, name));
                            generate_cfg_in_dot_format(&stbgr.functions[idx], filename, &stbgr);
                            function.cfg.as_ref().unwrap().display();
                        }
                    },
                    Some(Infos::IR) => {
                        println!("{}", stbgr);
                    },
                    Some(Infos::CompileModule) => {
                        println!("{:#?}", cm);
                    },
                    _ => {
                        continue;
                    }
                }
            },
            Some(Commands::Detection { detection }) => {
                // println!("{:?}",function.code);
                match *detection {
                    Some(Defects::UncheckedReturn) => {
                       stbgr.functions.iter().enumerate().map(|(idx, function)| {
                            if detect_unchecked_return(function) {
                                let name = cm.identifier_at(cm.function_handle_at(cm.function_defs[idx].function).name);
                                println!("{} : {}", name, "unchecked return");
                            }
                        }).for_each(drop);
                    },
                    Some(Defects::Overflow) => {
                        stbgr.functions.iter().enumerate().map(|(idx, function)| {
                        if detect_overflow(function) {
                                let name = cm.identifier_at(cm.function_handle_at(cm.function_defs[idx].function).name);
                                println!("{} : {}", name, "overflow");
                            }
                        }).for_each(drop);
                    },
                    Some(Defects::PrecisionLoss) => {
                        stbgr.functions.iter().enumerate().map(|(idx, function)| {
                            if detect_precision_loss(function, &stbgr.symbol_pool) {
                                let name = cm.identifier_at(cm.function_handle_at(cm.function_defs[idx].function).name);
                                println!("{} : {}", name, "precision loss");
                            }
                        }).for_each(drop);
                    },
                    Some(Defects::InfiniteLoop) => {
                        stbgr.functions.iter().enumerate().map(|(idx, function)| {
                            if detect_infinite_loop(&stbgr, idx) {
                                let name = cm.identifier_at(cm.function_handle_at(cm.function_defs[idx].function).name);
                                println!("{} : {}", name, "infinite loop");
                            }
                        }).for_each(drop);
                    },
                    Some(Defects::UnusedConstant) => {
                        let unused_constants = detect_unused_constants(&stbgr);
                        unused_constants.iter().map(|con| {
                            println!("{}", con);
                        }).for_each(drop);
                    },
                    Some(Defects::UnusedPrivateFunctions) => {
                        let unused_private_functions = detect_unused_private_functions(&stbgr);
                        unused_private_functions.iter().map(|func| {
                            println!("{}", func.symbol().display(&stbgr.symbol_pool));
                        }).for_each(drop);
                    },
                    Some(Defects::UnnecessaryTypeConversion) => {
                        let _ = stbgr.functions.iter().enumerate().map(|(idx, function)| {
                            if detect_unnecessary_type_conversion(function, &function.local_types) {
                                let name = cm.identifier_at(cm.function_handle_at(cm.function_defs[idx].function).name);
                                println!("{} : {}", name, "unnecessary type conversion");
                            }
                        }).for_each(drop);
                    },
                    Some(Defects::UnnecessaryBoolJudgment) => {
                        stbgr.functions.iter().enumerate().map(|(idx, function)| {
                            if detect_unnecessary_bool_judgment(function, &function.local_types) {
                                let name = cm.identifier_at(cm.function_handle_at(cm.function_defs[idx].function).name);
                                println!("{} : {}", name, "unnecessary bool judgment");
                            }
                        }).for_each(drop);
                    },
                    None => {
                        for (idx, function) in stbgr.functions.iter().enumerate() {
                            if detect_unchecked_return(function) {
                                let name = cm.identifier_at(cm.function_handle_at(cm.function_defs[idx].function).name);
                                println!("{} : {}", name, "unchecked return");
                            }
                            if detect_overflow(function) {
                                let name = cm.identifier_at(cm.function_handle_at(cm.function_defs[idx].function).name);
                                println!("{} : {}", name, "overflow");
                            }
                            if detect_precision_loss(function, &stbgr.symbol_pool) {
                                let name = cm.identifier_at(cm.function_handle_at(cm.function_defs[idx].function).name);
                                println!("{} : {}", name, "precision loss");
                            }
                            if detect_infinite_loop(&stbgr, idx) {
                                let name = cm.identifier_at(cm.function_handle_at(cm.function_defs[idx].function).name);
                                println!("{} : {}", name, "infinite loop");
                            }
                            if detect_unnecessary_type_conversion(function, &function.local_types) {
                                let name = cm.identifier_at(cm.function_handle_at(cm.function_defs[idx].function).name);
                                println!("{} : {}", name, "unnecessary type conversion");
                            }
                            if detect_unnecessary_bool_judgment(function, &function.local_types) {
                                let name = cm.identifier_at(cm.function_handle_at(cm.function_defs[idx].function).name);
                                println!("{} : {}", name, "unnecessary bool judgment");
                            }
                        }
                        let unused_constants = detect_unused_constants(&stbgr);
                        unused_constants.iter().map(|con| {
                            println!("{}", con);
                        }).for_each(drop);
                        let unused_private_functions = detect_unused_private_functions(&stbgr);
                        unused_private_functions.iter().map(|func| {
                            println!("{}", func.symbol().display(&stbgr.symbol_pool));
                        }).for_each(drop);
                    },
                    _ => {
                        println!("ERROR!");
                    }
                }
                println!(
                    "myapp detection was used for dealing with {}, name is: {:?}",
                    cli.filedir, detection
                )
            },
            None => {
                println!(
                    "no app was used for dealing with {}",
                    cli.filedir
                )
            }
        }
    }
}
