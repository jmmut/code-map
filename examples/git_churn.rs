use clap::Parser;
use code_map::git_churn::print_git_churn;
use code_map::AnyError;
use std::path::PathBuf;

/// Measure git churn: how many times each file has been changed.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// plot file sizes under this folder.
    #[arg(default_value = ".")]
    pub input_folder: PathBuf,
}

fn main() -> Result<(), AnyError> {
    print_git_churn(Cli::parse().input_folder)
}
