use std::path::PathBuf;

use clap::Parser;

/// Plot hierarchical metrics like file sizes in a folder structure.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// plot file sizes under this folder.
    #[arg(default_value = ".")]
    pub input_folder: PathBuf,

    /// Padding in pixels between hierarchies (e.g. 4).
    #[arg(short, long, default_value = "0")]
    pub padding: f32,

    /// arrangement algorithm: linear or square.
    #[arg(short, long, default_value = "square")]
    pub arrangement: String,
}

pub fn get_args() -> Cli {
    Cli::parse()
}
