mod bytes_per_file;
mod node;
mod treemap;

use macroquad::prelude::*;
use crate::treemap::MapNode;

type AnyError = Box<dyn std::error::Error>;

const DEFAULT_WINDOW_WIDTH: i32 = 1200;
const DEFAULT_WINDOW_HEIGHT: i32 = 675;
const DEFAULT_WINDOW_TITLE: &str = "Code Tree";

const FONT_SIZE: f32 = 16.0;

#[macroquad::main(window_conf)]
async fn main() -> Result<(), AnyError> {
    let args: Vec<String> = std::env::args().collect();
    let folder = if args.len() > 1 {
        args[1].clone()
    } else {
        ".".to_string()
    };
    let tree = bytes_per_file::bytes_per_file(&folder).unwrap();
    let units = "bytes";

    let mut treemap = MapNode::new(tree);
    let width = screen_width();
    let height = screen_height();
    let mut available = Rect::new(
        (width * 0.05).round(),
        (height * 0.05).round(),
        (width * 0.9).round(),
        (height * 0.75).round(),
    );
    treemap.arrange_top_level(available);
    let font_size = choose_font_size(width, height);
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        let width = screen_width();
        let height = screen_height();
        available = Rect::new(
            (width * 0.05).round(),
            (height * 0.05).round(),
            (width * 0.9).round(),
            (height * 0.75).round(),
        );
        clear_background(LIGHTGRAY);

        draw_rectangle_lines(
            (width * 0.05).round(),
            (height * 0.05).round(),
            available.w,
            available.h,
            1.0,
            BLACK,
        );

        // if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_position = Vec2::from(mouse_position());
        // let virtual_position = Vec2::new(
        //     (mouse_position.x - available.x) / available.w,
        //     (mouse_position.y - available.y) / available.h,
        // );
        // if Rect::new(0.0, 0.0, 1.0, 1.0).contains(virtual_position) {
        if available.contains(mouse_position) {
            let deepest_child = treemap.deepest_child(mouse_position);
            let text = format!("{}: {} {}", deepest_child.name, deepest_child.size, units);
            draw_text(
                &text,
                available.x + 1.5 * font_size,
                available.y + available.h + 5.0 * font_size,
                font_size,
                BLACK,
            );
        }
        // }
        for child in &treemap.children {
            draw_node(child, available, font_size);
        }
        next_frame().await
    }
    // println!("{:#?}", tree);
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

fn choose_font_size(width: f32, height: f32) -> f32 {
    let min_side = width.min(height * 16.0 / 9.0);
    FONT_SIZE
        * if min_side < 1600.0 {
            1.0
        } else if min_side < 2500.0 {
            1.5
        } else {
            2.0
        }
}

fn draw_node(node: &MapNode, mut available: Rect, font_size: f32) {


    let w = (node.rect.unwrap().w).round();
    let h = (node.rect.unwrap().h).round();
    let x = (node.rect.unwrap().x).round();
    let y = (node.rect.unwrap().y).round();
    draw_rectangle_lines(x, y, w, h, 1.0, BLACK);
    // draw_text(
    //     &node.name,
    //     x + 1.5 * font_size,
    //     y + 1.5 * font_size,
    //     font_size,
    //     BLACK,
    // );
    // draw_text(
    //     &node.size.to_string(),
    //     x + 1.5 * font_size,
    //     y + 3.0 * font_size,
    //     font_size,
    //     BLACK,
    // );
    for child in &node.children {
        draw_node(child, available, font_size);
    }
}
