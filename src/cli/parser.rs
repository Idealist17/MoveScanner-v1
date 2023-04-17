use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author="yule liteng", version="0.01", about="This is a static analysis tool for move smart contracts.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[clap(short = 'f', long, default_value_t = ("filedir").to_string(), help = "The project under this dir will be analyzed")]
    pub filedir: String,
}

#[derive(Subcommand)]
pub enum Commands {
    Printer { printer: Option<Infos> },
    Detection { detection:  Option<Defects> },
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Defects {
    BadAccessControl,
    UncheckedReturn,
    Overflow,
    PrecisionLoss,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Infos {
    IR,
    CompileModule,
    CFG,
    DefUse,
    FunctionVisibility,
}