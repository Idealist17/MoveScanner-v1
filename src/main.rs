#![allow(non_snake_case)]
use MoveScanner::{
    cli::parser::{Cli, SubCommands},
    scanner::{detectors::GraphGenerator, printer::Printer, option::Options},
};
use clap::Parser;
// use env_logger;
fn main() {
    // env_logger::init();
    let cli = Cli::parse();
    let option = Options::build_options(cli.args);
    match &cli.command {
        Some(SubCommands::Printer) => {
            // todo: 代码优化
            let mut printer = Printer::new(option);
            printer.run();
        }
        // Default: Graph Generation
        _ => {
            let mut generator = GraphGenerator::new(option);
            generator.run();
            // detector.output_result(); // Removed output.json generation
        }
    }
}
