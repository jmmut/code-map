use std::path::PathBuf;

use clap::Parser;

/// Plot hierarchical metrics like file sizes in a folder structure.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// plot file sizes under this folder.
    #[arg(default_value = ".")]
    input_folder: PathBuf,

    /// Padding in pixels between hierarchies (e.g. 4).
    #[arg(short, long, default_value = "0")]
    padding: f32,
}

pub fn get_args() -> (PathBuf, f32) {
    let args = Cli::parse();
    let folder = args.input_folder;
    let padding = args.padding;
    (folder, padding)
}
