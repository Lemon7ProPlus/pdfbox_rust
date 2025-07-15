use std::path::PathBuf;
use clap::{Args, Parser, Subcommand, ValueHint};


pub mod prelude {
    pub use super::CommandResult;
    pub use super::parse_args;
}

#[derive(Debug)]
pub enum CommandResult {
    Merge {
        input: Vec<PathBuf>,
        exclude_files: Vec<PathBuf>,
        toc_file: Option<PathBuf>,
        name: Option<String>,
        output: Option<PathBuf>,
    },
    
    ExtractTitle {
        input: Vec<PathBuf>,
        toc_output: Option<PathBuf>,
    },
}

pub fn parse_args() -> CommandResult {
    let cli = Cli::parse();
    match cli.command {
        Commands::Merge(args) => CommandResult::Merge { 
            input: args.input, 
            exclude_files: args.exclude_files, 
            toc_file: args.toc_file,
            name: args.name, 
            output: args.output, 
        },

        Commands::ExtractTitle(args) => CommandResult::ExtractTitle { 
            input: args.input, 
            toc_output: args.toc_output, 
        },
    }
}

// Clap Define
#[derive(Parser, Debug)]
#[command(
    name    = "PDF toolbox",
    author  = "Time",
    version = env!("CARGO_PKG_VERSION"),
    about   = "Merger PDFs and extract titles from PDFs"    
)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Merge(MergeArgs),

    ExtractTitle(ExtractTitleArgs),
}

#[derive(Args, Debug)]
struct MergeArgs {
    #[arg(short= 'i', long="input", value_name = "INPUT", num_args = 1.., value_hint = ValueHint::AnyPath)]
    input: Vec<PathBuf>,

    #[arg(short = 'e', long="exclude", value_name = "EXCLUDE_FILES", num_args = 1.., value_hint = ValueHint::FilePath)]
    exclude_files: Vec<PathBuf>,

    #[arg(short = 't', long="toc_input", value_name = "TOC_FILE", value_hint = ValueHint::FilePath)]
    toc_file: Option<PathBuf>,

    #[arg(short = 'n', long="name", value_name = "Name", value_hint = ValueHint::FilePath)]
    name: Option<String>,

    #[arg(short = 'o', long="output", value_name = "OUTPUT", value_hint = ValueHint::FilePath)]
    output: Option<PathBuf>,
    
}

#[derive(Args, Debug)]
struct ExtractTitleArgs {
    #[arg(short= 'i', long="input", value_name = "INPUT", num_args = 1.., value_hint = ValueHint::AnyPath)]
    input: Vec<PathBuf>,

    #[arg(short = 'o', long="toc_output", value_name = "TOC_OUTPUT", value_hint = ValueHint::FilePath)]
    toc_output: Option<PathBuf>,
    
}