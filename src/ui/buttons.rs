use crate::ui::rect_utils::draw_rect;
use crate::ui::{button_height, button_margin, small_pad};
use juquad::input::input_macroquad::InputMacroquad;
use juquad::input::input_trait::InputTrait;
use juquad::widgets::Interaction;
use juquad::widgets::anchor::{Anchor, Horizontal, Layout, Vertical};
use macroquad::color::{BLACK, DARKGRAY, GRAY, WHITE};
use macroquad::math::{Rect, Vec2, vec2};
use macroquad::prelude::{draw_text, measure_text};

pub struct Buttons {
    pub copy_to_clipboard: Button,
    pub refresh: Button,
}

pub const TO_LEFT: Layout = Layout::Horizontal {
    direction: Horizontal::Left,
    alignment: Vertical::Top,
};

impl Buttons {
    pub fn new(panel: Rect, font_size: f32) -> Self {
        let mut ui = Layouter::new(panel, TO_LEFT, font_size);

        let copy_to_clipboard = ui.button("Copy to clipboard");
        let refresh = ui.button("Refresh");
        // let squareness = ui.button("Squareness");

        Self {
            copy_to_clipboard,
            refresh,
        }
    }
    pub fn rect(&self) -> Rect {
        self.refresh.rect.combine_with(self.copy_to_clipboard.rect)
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
        self.next_anchor =
            Anchor::next_to(new_button.rect, self.layout, button_margin(self.font_size));
        new_button
    }
}

pub struct Button {
    pub text: String,
    pub rect: Rect,
    pub horizontal_pad: f32,
    pub font_size: f32,
    pub interaction: Interaction,
}

#[allow(unused)]
pub fn immediate_button(text: &str, anchor: Anchor, font_size: f32) -> (Rect, bool) {
    let mut button_ = button(text, anchor, font_size);
    interact(&mut button_);
    draw_button(&button_);
    (button_.rect, button_.interaction.is_clicked())
}

pub fn button(text: &str, anchor: Anchor, font_size: f32) -> Button {
    let horizontal_pad = font_size * 1.0;
    let measure = measure_text(text, None, font_size as u16, 1.0);
    let size = vec2(
        measure.width + horizontal_pad * 2.0,
        button_height(font_size),
    );
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
