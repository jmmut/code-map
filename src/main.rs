use std::path::PathBuf;

use clap::Parser;
use git_version::git_version;
use macroquad::prelude::*;

use code_map::arrangements::{binary, linear};
use code_map::metrics::word_mentions::TEXT_FILE_EXTENSIONS;
use code_map::metrics::Metrics;
use code_map::tree::Tree;
use code_map::ui::Ui;
use code_map::{metrics, AnyError};

const DEFAULT_WINDOW_WIDTH: i32 = 1200;
const DEFAULT_WINDOW_HEIGHT: i32 = 675;
const DEFAULT_WINDOW_TITLE: &str = "Code Map";

pub const GIT_VERSION: &str = git_version!(args = ["--tags"]);

/// Plot hierarchical metrics like file sizes in a folder structure.
#[derive(Parser, Clone)]
#[command(author, version = GIT_VERSION, about, long_about = None)]
pub struct Cli {
    /// plot file sizes under this folder.
    #[arg(default_value = ".")]
    pub input_folder: PathBuf,

    /// arrangement algorithm: linear or binary.
    #[arg(short, long, default_value = "binary")]
    pub arrangement: String,

    /// metric to plot
    #[arg(short, long, default_value = "churn-per-file")]
    pub metric: Metrics,

    // Don't filter by extension of source code files
    // #[arg(short = 'x', long, default_value = false)]
    // pub all_extensions: bool,
    //
    /// Padding in pixels between hierarchies (e.g. 4) (only for linear arrangement).
    #[arg(short, long, default_value = "0")]
    pub padding: f32,

    /// maximum number of commits to consider (only for churn-per-file metric)
    #[arg(long)]
    pub max_commits: Option<usize>,
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
    let args = Cli::parse();
    let mut ui = compute_ui(args.clone());
    while should_continue() {
        if ui.should_refresh() {
            ui = compute_ui(args.clone());
        }
        ui.draw();
        next_frame().await
    }
    Ok(())
}

fn compute_ui(args: Cli) -> Ui {
    let all_extensions = true;
    let Cli {
        input_folder,
        padding,
        arrangement,
        metric,
        // all_extensions,
        max_commits,
    } = args;
    let (tree, units) = log_time!(
        compute_metrics(&input_folder, &metric, all_extensions, max_commits),
        format!("computing metrics {:?}", metric)
    );

    let mut ui = Ui::new(tree, units, arrange, arrangement.clone(), padding);
    log_time!(
        arrange(padding, arrangement.clone(), &mut ui.tree, ui.map_rect),
        "arrangement"
    );
    log_time!(log_counts(&ui.tree));
    ui
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
    metric: &Metrics,
    all_extensions: bool,
    max_commits: Option<usize>,
) -> (Tree, &'static str) {
    let (tree, units) = match metric {
        Metrics::BytesPerFile => (
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
        Metrics::WordMentions => (
            metrics::word_mentions::word_mentions(&input_folder).unwrap(),
            "mentions",
        ),
        Metrics::LinesPerFile => (
            metrics::lines::lines_per_file(&input_folder)
                .unwrap()
                .unwrap(),
            "lines",
        ),
        Metrics::ChurnPerFile => (
            metrics::churn_per_file::git_churn_per_file(input_folder.clone(), max_commits).unwrap(),
            "modifications (commits per file)",
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
    // TODO: squareness is broken
    // info!(
    //     "The arrangement has a squareness of {}",
    //     treemap.compute_squareness()
    // );
}
