use clap::Parser;
use macroquad::prelude::*;
use std::path::PathBuf;

use code_map::arrangements::{binary, linear};
use code_map::metrics::word_mentions::TEXT_FILE_EXTENSIONS;
use code_map::tree::Tree;
use code_map::ui::Ui;
use code_map::{metrics, AnyError};

const DEFAULT_WINDOW_WIDTH: i32 = 1200;
const DEFAULT_WINDOW_HEIGHT: i32 = 675;
const DEFAULT_WINDOW_TITLE: &str = "Code Map";

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

    /// arrangement algorithm: linear or binary.
    #[arg(short, long, default_value = "binary")]
    pub arrangement: String,

    /// metric to plot: bytes-per-file (or b), mentions-per-word (or w), lines-per-file (or l).
    #[arg(short, long, default_value = "lines-per-file")]
    pub metric: String,
    // Don't filter by extension of source code files
    // #[arg(short = 'x', long, default_value = false)]
    // pub all_extensions: bool,
}

macro_rules! log_time {
    ($e:expr $(,)?) => {{
        let time_before = std::time::Instant::now();
        let result = $e;
        let time_after = std::time::Instant::now();
        info!("{} took {:?}", stringify!($e), time_after - time_before);
        result
    }};
    ($e:expr, $name:expr $(,)?) => {{
        let time_before = std::time::Instant::now();
        let result = $e;
        let time_after = std::time::Instant::now();
        info!("{} took {:?}", $name, time_after - time_before);
        result
    }};
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), AnyError> {
    let all_extensions = true;
    let Cli {
        input_folder,
        padding,
        arrangement,
        metric,
        // all_extensions,
    } = Cli::parse();

    let (tree, units) = log_time!(
        compute_metrics(&input_folder, &metric, all_extensions),
        "computing metrics"
    );

    let mut ui = Ui::new(tree, units);
    log_time!(
        arrange(padding, arrangement, &mut ui.tree, ui.available),
        "arrangement"
    );
    log_time!(log_counts(&ui.tree));


    while should_continue() {
        ui.draw();
        next_frame().await
    }
    Ok(())
}

fn should_continue() -> bool {
    let ctrl_q_pressed = is_key_pressed(KeyCode::Q)
        && (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl));
    let escape_pressed = is_key_down(KeyCode::Escape);
    let should_quit = ctrl_q_pressed || escape_pressed;
    !should_quit
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        high_dpi: true,
        ..Default::default()
    }
}

fn compute_metrics(
    input_folder: &PathBuf,
    metric: &str,
    all_extensions: bool,
) -> (Tree, &'static str) {
    let (tree, units) = match metric {
        "bytes-per-file" | "b" => (
            if all_extensions {
                metrics::bytes_per_file::bytes_per_file(&input_folder).unwrap()
            } else {
                metrics::bytes_per_file::bytes_per_file_with_extension(
                    &input_folder,
                    TEXT_FILE_EXTENSIONS,
                )
                .unwrap()
                .unwrap()
            },
            "bytes",
        ),
        "mentions-per-word" | "w" => (
            metrics::word_mentions::word_mentions(&input_folder).unwrap(),
            "mentions",
        ),
        "lines-per-file" | "l" => (
            metrics::lines::lines_per_file(&input_folder)
                .unwrap()
                .unwrap(),
            "lines",
        ),
        _ => panic!(
            "Unknown metric: {}. valid ones are: bytes-per-file, b, mentions-per-word, w",
            metric
        ),
    };
    (tree, units)
}

fn arrange(padding: f32, arrangement: String, mut treemap: &mut Tree, available: Rect) {
    if arrangement == "linear" {
        linear::arrange(&mut treemap, available, padding);
    } else if arrangement == "binary" {
        binary::arrange(&mut treemap, available);
    } else {
        panic!(
            "Unknown arrangement algorithm: {}. valid ones are: linear, binary",
            arrangement
        );
    }
}

fn log_counts(treemap: &Tree) {
    let counts = treemap.count();
    info!(
        "There are {} items. {} including the hierarchy levels",
        counts.leafs, counts.total
    );
    let visible_counts = treemap.count_visible();
    if visible_counts.total != counts.total || visible_counts.leafs != counts.leafs {
        info!(
            "However, only {} items and {} items+hierarchies are visible",
            visible_counts.leafs, visible_counts.total
        );
    }
    info!("The arrangement has a squareness of {}", treemap.compute_squareness());
}
