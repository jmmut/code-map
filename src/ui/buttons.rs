use crate::ui::rect_utils::draw_rect;
use macroquad::color::{BLACK, DARKGRAY, GRAY, WHITE};
use macroquad::input::mouse_position;
use macroquad::math::{Rect, Vec2};
use macroquad::prelude::{
    draw_text, is_mouse_button_down, is_mouse_button_pressed, measure_text, screen_height,
    MouseButton,
};

pub struct PressedButtons {
    pub refresh: bool,
    pub copied: bool,
    pub squareness: bool,
}

pub fn draw_buttons(map_rect: Rect, font_size: f32) -> PressedButtons {
    let y = screen_height() - font_size * 3.5;
    let (button_rect, copied) = draw_button("Copy to clipboard", map_rect.x, y, font_size);

    let next_x = button_rect.x + button_rect.w + font_size;
    let (_button_rect, refresh) = draw_button("Refresh", next_x, y, font_size);

    // let next_x = button_rect.x + button_rect.w + font_size;
    // let (_button_rect, squareness) = draw_button("Compute squareness", next_x, y, font_size);
    PressedButtons {
        refresh,
        copied,
        squareness: false,
    }
}

fn draw_button(text: &str, x: f32, y: f32, font_size: f32) -> (Rect, bool) {
    let horizontal_pad = font_size * 1.0;
    let mut pressed = false;

    let measure = measure_text(text, None, font_size as u16, 1.0);
    let button_rect = Rect::new(x, y, measure.width + horizontal_pad * 2.0, font_size * 1.5);
    if button_rect.contains(Vec2::from(mouse_position())) {
        if is_mouse_button_pressed(MouseButton::Left) {
            pressed = true;
        }
        if is_mouse_button_down(MouseButton::Left) {
            draw_rect(button_rect, BLACK);
        } else {
            draw_rect(button_rect, DARKGRAY);
        }
    } else {
        draw_rect(button_rect, GRAY);
    }
    draw_text(
        text,
        button_rect.x + horizontal_pad,
        button_rect.y + font_size,
        font_size,
        WHITE,
    );
    (button_rect, pressed)
}
