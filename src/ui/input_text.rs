use crate::ui::key_queue::InputCharacter;
use macroquad::input::KeyCode;
use macroquad::prelude::{draw_rectangle, draw_rectangle_lines, draw_text, Rect, BLACK, WHITE};
use std::collections::VecDeque;

pub struct InputText<'a> {
    pub rect: Rect,
    pub text: &'a mut String,
    pub keys: &'a VecDeque<InputCharacter>,
    pub focused: bool,
    pub font_size: f32,
}
impl<'a> InputText<'a> {
    pub fn new(
        rect: Rect,
        text: &'a mut String,
        keys: &'a VecDeque<InputCharacter>,
        font_size: f32,
    ) -> Self {
        Self {
            rect,
            text,
            keys,
            font_size,
            focused: false,
        }
    }
}

impl<'a> InputText<'a> {
    pub fn interact(&mut self, focused: bool) {
        self.focused = focused;
        if self.focused {
            for InputCharacter { key, modifier } in self.keys {
                if !modifier.alt && !modifier.ctrl && !modifier.shift && !modifier.logo {
                    self.interact_no_modifiers(*key);
                }
            }
        }
    }
    pub fn render(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
        if self.focused {
            draw_rectangle_lines(
                self.rect.x,
                self.rect.y,
                self.rect.w,
                self.rect.h,
                1.0,
                BLACK,
            );
        }
        draw_text(
            &self.text,
            self.rect.x + self.font_size * 0.5,
            self.rect.y + self.font_size,
            self.font_size,
            BLACK,
        );
    }
}

impl<'a> InputText<'a> {
    fn interact_no_modifiers(&mut self, key: KeyCode) {
        match key {
            KeyCode::Space => {}
            KeyCode::Apostrophe => {}
            KeyCode::Comma => {}
            KeyCode::Minus => {}
            KeyCode::Period => {}
            KeyCode::Slash => {}
            KeyCode::Key0 => {}
            KeyCode::Key1 => {}
            KeyCode::Key2 => {}
            KeyCode::Key3 => {}
            KeyCode::Key4 => {}
            KeyCode::Key5 => {}
            KeyCode::Key6 => {}
            KeyCode::Key7 => {}
            KeyCode::Key8 => {}
            KeyCode::Key9 => {}
            KeyCode::Semicolon => {}
            KeyCode::Equal => {}
            KeyCode::A
            | KeyCode::B
            | KeyCode::C
            | KeyCode::D
            | KeyCode::E
            | KeyCode::F
            | KeyCode::G
            | KeyCode::H
            | KeyCode::I
            | KeyCode::J
            | KeyCode::K
            | KeyCode::L
            | KeyCode::M
            | KeyCode::N
            | KeyCode::O
            | KeyCode::P
            | KeyCode::Q
            | KeyCode::R
            | KeyCode::S
            | KeyCode::T
            | KeyCode::U
            | KeyCode::V
            | KeyCode::W
            | KeyCode::X
            | KeyCode::Y
            | KeyCode::Z => {
                self.text
                    .push((key as u8 - KeyCode::A as u8 + 'a' as u8) as char);
            }
            KeyCode::LeftBracket => {}
            KeyCode::Backslash => {}
            KeyCode::RightBracket => {}
            KeyCode::GraveAccent => {}
            KeyCode::World1 => {}
            KeyCode::World2 => {}
            KeyCode::Escape => {}
            KeyCode::Enter => {}
            KeyCode::Tab => {}
            KeyCode::Backspace => {
                self.text.pop();
            }
            KeyCode::Insert => {}
            KeyCode::Delete => {}
            KeyCode::Right => {}
            KeyCode::Left => {}
            KeyCode::Down => {}
            KeyCode::Up => {}
            KeyCode::PageUp => {}
            KeyCode::PageDown => {}
            KeyCode::Home => {}
            KeyCode::End => {}
            KeyCode::CapsLock => {}
            KeyCode::ScrollLock => {}
            KeyCode::NumLock => {}
            KeyCode::PrintScreen => {}
            KeyCode::Pause => {}
            KeyCode::F1 => {}
            KeyCode::F2 => {}
            KeyCode::F3 => {}
            KeyCode::F4 => {}
            KeyCode::F5 => {}
            KeyCode::F6 => {}
            KeyCode::F7 => {}
            KeyCode::F8 => {}
            KeyCode::F9 => {}
            KeyCode::F10 => {}
            KeyCode::F11 => {}
            KeyCode::F12 => {}
            KeyCode::F13 => {}
            KeyCode::F14 => {}
            KeyCode::F15 => {}
            KeyCode::F16 => {}
            KeyCode::F17 => {}
            KeyCode::F18 => {}
            KeyCode::F19 => {}
            KeyCode::F20 => {}
            KeyCode::F21 => {}
            KeyCode::F22 => {}
            KeyCode::F23 => {}
            KeyCode::F24 => {}
            KeyCode::F25 => {}
            KeyCode::Kp0 => {}
            KeyCode::Kp1 => {}
            KeyCode::Kp2 => {}
            KeyCode::Kp3 => {}
            KeyCode::Kp4 => {}
            KeyCode::Kp5 => {}
            KeyCode::Kp6 => {}
            KeyCode::Kp7 => {}
            KeyCode::Kp8 => {}
            KeyCode::Kp9 => {}
            KeyCode::KpDecimal => {}
            KeyCode::KpDivide => {}
            KeyCode::KpMultiply => {}
            KeyCode::KpSubtract => {}
            KeyCode::KpAdd => {}
            KeyCode::KpEnter => {}
            KeyCode::KpEqual => {}
            KeyCode::LeftShift => {}
            KeyCode::LeftControl => {}
            KeyCode::LeftAlt => {}
            KeyCode::LeftSuper => {}
            KeyCode::RightShift => {}
            KeyCode::RightControl => {}
            KeyCode::RightAlt => {}
            KeyCode::RightSuper => {}
            KeyCode::Menu => {}
            KeyCode::Unknown => {}
        }
    }
}
