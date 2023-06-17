use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use MoveScanner::{cli::parser::*, 
    move_ir::{sbir_generator::{MoveScanner as Mc, Blockchain}, generate_bytecode::StacklessBytecodeGenerator}, utils::utils::compile_module, detect::{detect3::detect_precision_loss, detect1::detect_unchecked_return, detect7::detect_unnecessary_type_conversion, detect8::detect_unnecessary_bool_judgment},
};
use move_binary_format::access::ModuleAccess;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Printer { printer }) => {
            let dir = "./testdata/examples_mv/aptos/";
            let bc = Blockchain::Aptos;
            let ms = Mc::new(dir, bc);
            match printer {
                Some(Infos::CFG) => {
                    let (qid, _) = ms.functions.first_key_value().unwrap();
                    let file_path = Some(PathBuf::from("cfg.dot"));
                    let _ = ms.get_cfg(qid, file_path);
                },
                Some(Infos::CompileModule) => {
                    let (qid, _) = ms.functions.first_key_value().unwrap();
                }, 
                Some(Infos::IR) => {
                    let mut text = String::new();
                    text += &ms.print_targets_for_test();
                    println!("{}", text);
                },
                _ => {

                },
            }
        },
        Some(Commands::Detection { detection }) => {
            let filename = PathBuf::from_str(&cli.filedir).unwrap();
            let cm = compile_module(filename);
            let mut stbgr = StacklessBytecodeGenerator::new(&cm);
            stbgr.generate_function();
            for (idx, function) in stbgr.functions.iter().enumerate() {
                // println!("{:?}",function.code);
                match *detection {
                    Some(Defects::UncheckedReturn) => {
                        if detect_unchecked_return(function) {
                            let name = cm.identifier_at(cm.function_handle_at(cm.function_defs[idx].function).name);
                            println!("{} : {}", name, "unchecked return");
                        }
                    },
                    Some(Defects::Overflow) => {
                        println!("TODO");
                    },
                    Some(Defects::PrecisionLoss) => {
                        if detect_precision_loss(function, &stbgr.symbol_pool) {
                            let name = cm.identifier_at(cm.function_handle_at(cm.function_defs[idx].function).name);
                            println!("{} : {}", name, "precision loss");
                        }
                    },
                    Some(Defects::InfiniteLoop) => {
                        println!("TODO");
                    },
                    Some(Defects::UnusedConstant) => {
                        println!("TODO");
                    },
                    Some(Defects::UnusedPrivateFunctions) => {
                        println!("TODO");
                    },
                    Some(Defects::UnnecessaryTypeConversion) => {
                        if detect_unnecessary_type_conversion(function, &function.local_types) {
                            let name = cm.identifier_at(cm.function_handle_at(cm.function_defs[idx].function).name);
                            println!("{} : {}", name, "unnecessary type conversion");
                        }
                    },
                    Some(Defects::UnnecessaryBoolJudgment) => {
                        if detect_unnecessary_bool_judgment(function, &function.local_types) {
                            let name = cm.identifier_at(cm.function_handle_at(cm.function_defs[idx].function).name);
                            println!("{} : {}", name, "unnecessary bool judgment");
                        }
                    },
                    None => {
                        println!("ERROR");
                    },
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
