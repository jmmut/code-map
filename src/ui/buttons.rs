use crate::ui::rect_utils::draw_rect;
use crate::ui::small_pad;
use juquad::input::input_macroquad::InputMacroquad;
use juquad::input::input_trait::InputTrait;
use juquad::widgets::Interaction;
use juquad::widgets::anchor::{Anchor, Horizontal, Layout, Vertical};
use macroquad::color::{BLACK, DARKGRAY, GRAY, WHITE};
use macroquad::input::mouse_position;
use macroquad::math::{Rect, Vec2, vec2};
use macroquad::prelude::{
    MouseButton, draw_text, is_mouse_button_down, is_mouse_button_released, measure_text,
    screen_height, screen_width,
};

pub struct PressedButtons {
    pub refresh: bool,
    pub copied: bool,
    pub squareness: bool,
}

pub struct Buttons {
    pub copy_to_clipboard: Button,
    pub refresh: Button,
}

impl Buttons {
    pub fn new(screen: Vec2, font_size: f32) -> Self {
        let screen_rect = Rect::new(0.0, 0.0, screen.x, screen.y);
        let layout = Layout::Horizontal {
            direction: Horizontal::Left,
            alignment: Vertical::Top,
        };
        let mut ui = Layouter::new(screen_rect, layout, font_size);

        let copy_to_clipboard = ui.button("Copy to clipboard");
        let refresh = ui.button("Refresh");
        // let squareness = ui.button("Squareness");

        Self {
            copy_to_clipboard,
            refresh,
        }
    }
    pub fn draw(&self) {
        draw_button(&self.copy_to_clipboard);
        draw_button(&self.refresh);
    }
}

pub struct Layouter {
    pub layout: Layout,
    pub font_size: f32,
    pub next_anchor: Anchor,
}
impl Layouter {
    pub fn new(outer_rect: Rect, layout: Layout, font_size: f32) -> Self {
        let next_anchor = Anchor::inside(outer_rect, layout, Vec2::splat(small_pad(font_size)));
        Self {
            layout,
            font_size,
            next_anchor,
        }
    }
    pub fn button(&mut self, text: &str) -> Button {
        let new_button = button(text, self.next_anchor, self.font_size);
        self.next_anchor = Anchor::next_to(new_button.rect, self.layout, self.font_size);
        new_button
    }
}

pub struct Button {
    text: String,
    rect: Rect,
    horizontal_pad: f32,
    font_size: f32,
    interaction: Interaction,
}

pub fn immediate_button(text: &str, anchor: Anchor, font_size: f32) -> (Rect, bool) {
    let mut button_ = button(text, anchor, font_size);
    interact(&mut button_);
    draw_button(&button_);
    (button_.rect, button_.interaction.is_clicked())
}

pub fn button(text: &str, anchor: Anchor, font_size: f32) -> Button {
    let horizontal_pad = font_size * 1.0;
    let measure = measure_text(text, None, font_size as u16, 1.0);
    let size = vec2(measure.width + horizontal_pad * 2.0, font_size * 1.5);
    let button_rect = anchor.get_rect(size);
    Button {
        text: text.to_string(),
        rect: button_rect,
        horizontal_pad,
        font_size,
        interaction: Interaction::None,
    }
}
pub fn interact(button: &mut Button) -> Interaction {
    let input: Box<dyn InputTrait> = Box::new(InputMacroquad);
    let interaction = juquad::widgets::interact(button.rect, &input);
    button.interaction = interaction;
    button.interaction
}

pub fn draw_button(button: &Button) {
    match button.interaction {
        Interaction::Clicked | Interaction::None => {
            draw_rect(button.rect, GRAY);
        }
        Interaction::Hovered => {
            draw_rect(button.rect, DARKGRAY);
        }
        Interaction::Pressing => {
            draw_rect(button.rect, BLACK);
        }
    }

    draw_text(
        &button.text,
        button.rect.x + button.horizontal_pad,
        button.rect.y + button.font_size,
        button.font_size,
        WHITE,
    );
}

pub fn draw_buttons(map_rect: Rect, font_size: f32) -> PressedButtons {
    let layout = Layout::Horizontal {
        direction: Horizontal::Left,
        alignment: Vertical::Top,
    };
    let anchor = Anchor::inside(
        Rect::new(0.0, 0.0, screen_width(), screen_height()),
        layout,
        Vec2::splat(small_pad(font_size)),
    );

    let (_button_rect, copied) = immediate_button("Copy to clipboard", anchor, font_size);

    let anchor = Anchor::next_to(_button_rect, layout, font_size);
    let (_button_rect, refresh) = immediate_button("Refresh", anchor, font_size);

    // let next_x = button_rect.x + button_rect.w + font_size;
    // let (_button_rect, squareness) = draw_button("Compute squareness", next_x, y, font_size);
    PressedButtons {
        refresh,
        copied,
        squareness: false,
    }
}

pub fn draw_buttons2(font_size: f32) -> PressedButtons {
    let layout = Layout::Horizontal {
        direction: Horizontal::Left,
        alignment: Vertical::Top,
    };
    let screen_rect = Rect::new(0.0, 0.0, screen_width(), screen_height());
    let anchor = Anchor::inside(screen_rect, layout, Vec2::splat(small_pad(font_size)));

    let (_button_rect, copied) = immediate_button("Copy to clipboard", anchor, font_size);

    let anchor = Anchor::next_to(_button_rect, layout, font_size);
    let (_button_rect, refresh) = immediate_button("Refresh", anchor, font_size);

    // let next_x = button_rect.x + button_rect.w + font_size;
    // let (_button_rect, squareness) = draw_button("Compute squareness", next_x, y, font_size);
    PressedButtons {
        refresh,
        copied,
        squareness: false,
    }
}
