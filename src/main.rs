mod node;
mod bytes_per_file;

use macroquad::prelude::*;

type AnyError = Box<dyn std::error::Error>;

const DEFAULT_WINDOW_WIDTH: i32 = 1200;
const DEFAULT_WINDOW_HEIGHT: i32 = 675;
const DEFAULT_WINDOW_TITLE: &str = "Code Tree";

#[macroquad::main(window_conf)]
async fn main() -> Result<(), AnyError> {
    let args: Vec<String> = std::env::args().collect();
    let folder = if args.len() > 1 {
        args[1].clone()
    } else {
        ".".to_string()
    };
    let tree = bytes_per_file::bytes_per_file(&folder).unwrap();
    // loop {
    //     if is_key_pressed(KeyCode::Escape) {
    //         break;
    //     }
    //     clear_background(LIGHTGRAY);
    //     let width = screen_width();
    //     let height = screen_height();
    //     draw_rectangle_lines(width * 0.05, height * 0.05, width * 0.9, height * 0.75, 1.0, BLACK);
    //
    //     next_frame().await
    // }
    println!("{:#?}", tree);
    Ok(())
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
