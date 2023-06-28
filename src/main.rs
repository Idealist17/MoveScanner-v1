use std::{path::PathBuf, str::FromStr, fmt::format, fs, vec};

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
use itertools::Itertools;
use move_binary_format::{access::ModuleAccess, CompiledModule};

fn main() {
    let cli = Cli::parse();
    let dir = PathBuf::from(&cli.filedir);
    let mut paths = Vec::new();
    utils::visit_dirs(&dir, &mut paths, false);

    for filename in paths {
        let cm = compile_module(filename.clone());
        let mut stbgr = StacklessBytecodeGenerator::new(&cm);
        stbgr.generate_function();
        stbgr.get_control_flow_graph();
        stbgr.build_call_graph();
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
                println!("============== Handling for {:?} ==============", filename.to_str().unwrap());
                let mut detects: Vec<Vec<usize>> = vec![Vec::new(); 6];
                match *detection {
                    Some(Defects::UncheckedReturn) => {
                       stbgr.functions.iter().enumerate().map(|(idx, function)| {
                            if detect_unchecked_return(function, &stbgr.symbol_pool, idx, &cm) {
                                detects[0].push(idx);
                            }
                        }).for_each(drop);
                    },
                    Some(Defects::Overflow) => {
                        stbgr.functions.iter().enumerate().map(|(idx, function)| {
                        if detect_overflow(function) {
                                detects[1].push(idx);
                            }
                        }).for_each(drop);
                    },
                    Some(Defects::PrecisionLoss) => {
                        stbgr.functions.iter().enumerate().map(|(idx, function)| {
                            if detect_precision_loss(function, &stbgr.symbol_pool) {
                                detects[2].push(idx);
                            }
                        }).for_each(drop);
                    },
                    Some(Defects::InfiniteLoop) => {
                        stbgr.functions.iter().enumerate().map(|(idx, function)| {
                            if detect_infinite_loop(&stbgr, idx) {
                                detects[3].push(idx);
                            }
                        }).for_each(drop);
                    },
                    Some(Defects::UnnecessaryTypeConversion) => {
                        let _ = stbgr.functions.iter().enumerate().map(|(idx, function)| {
                            if detect_unnecessary_type_conversion(function, &function.local_types) {
                                detects[4].push(idx);
                            }
                        }).for_each(drop);
                    },
                    Some(Defects::UnnecessaryBoolJudgment) => {
                        stbgr.functions.iter().enumerate().map(|(idx, function)| {
                            if detect_unnecessary_bool_judgment(function, &function.local_types) {
                                detects[5].push(idx);
                            }
                        }).for_each(drop);
                    },
                    Some(Defects::UnusedConstant) => {
                        let unused_constants = detect_unused_constants(&stbgr);
                        println!("Unused constants: {:?}", unused_constants);
                    },
                    Some(Defects::UnusedPrivateFunctions) => {
                        let unused_private_functions = detect_unused_private_functions(&stbgr);
                        let unused_private_function_names = unused_private_functions.iter().map(|func| {
                            func.symbol().display(&stbgr.symbol_pool).to_string()
                        }).collect_vec();
                        println!("Unused private functions: {:?}", unused_private_function_names);
                    },
                    None => {
                        for (idx, function) in stbgr.functions.iter().enumerate() {
                            if detect_unchecked_return(function, &stbgr.symbol_pool, idx, &cm) {
                                detects[0].push(idx);
                            }
                            if detect_overflow(function) {
                                detects[1].push(idx);
                            }
                            if detect_precision_loss(function, &stbgr.symbol_pool) {
                                detects[2].push(idx);
                            }
                            if detect_infinite_loop(&stbgr, idx) {
                                detects[3].push(idx);

                            }
                            if detect_unnecessary_type_conversion(function, &function.local_types) {
                                detects[4].push(idx);
                            }
                            if detect_unnecessary_bool_judgment(function, &function.local_types) {
                                detects[5].push(idx);
                            }
                        }
                        let unused_constants = detect_unused_constants(&stbgr);
                        println!("Unused constants: {:?}", unused_constants);
                        let unused_private_functions = detect_unused_private_functions(&stbgr);
                        let unused_private_function_names = unused_private_functions.iter().map(|func| {
                            func.symbol().display(&stbgr.symbol_pool).to_string()
                        }).collect_vec();
                        println!("Unused private functions: {:?}", unused_private_function_names);
                    },
                }
                format_result(&detects, &cm);
                println!("==============================================\n");
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

fn format_result(detects: &Vec<Vec<usize>>, cm: &CompiledModule) {
    let detect_types = [
        "Unchecked return",
        "Overflow",
        "Precision loss",
        "Infinite loop",
        "Unnecessary type conversion",
        "Unnecessary bool judgment",
    ];
    for (i, d_type) in detect_types.iter().enumerate() {
        if detects[i].len() == 0 {
            continue;
        }
        let detect_fname = detects[i].iter().map(|idx| {
            let handle = cm.function_handle_at(cm.function_defs[*idx].function);
            cm.identifier_at(handle.name).as_str()
        }).collect_vec();
        println!("{}: {:?}", *d_type, detect_fname);
    }
}
