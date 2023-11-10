use macroquad::prelude::{is_mouse_button_pressed, mouse_position, MouseButton, Rect, Vec2};

/// I think macroquad will draw blurry pixels if the position or size of a rectangle is not rounded.
pub fn round_rect(rect: Rect) -> Rect {
    let rounded_x = rect.x.round();
    let rounded_y = rect.y.round();
    Rect::new(
        rounded_x,
        rect.y.round(),
        (rect.x + rect.w).round() - rounded_x,
        (rect.y + rect.h).round() - rounded_y,
    )
}

pub fn is_rect_clicked(rect: &Rect, mouse_button: MouseButton) -> bool {
    is_mouse_button_pressed(mouse_button) && rect.contains(Vec2::from(mouse_position()))
}
