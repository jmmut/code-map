use std::collections::VecDeque;

use macroquad::miniquad;
use macroquad::miniquad::{Context, GraphicsContext, KeyMods};
use macroquad::prelude::utils::{register_input_subscriber, repeat_all_miniquad_input};
use macroquad::prelude::KeyCode;

pub struct OrderedEventHandler {
    pub keycode_event_queue: VecDeque<InputCharacter>,
    miniquad_handler_id: usize,
}

pub struct InputCharacter {
    pub key: KeyCode,
    pub modifier: KeyMods,
}

impl OrderedEventHandler {
    pub fn new() -> OrderedEventHandler {
        let miniquad_handler_id = register_input_subscriber();
        OrderedEventHandler {
            keycode_event_queue: VecDeque::new(),
            miniquad_handler_id,
        }
    }
    pub fn capture_keys_this_frame(&mut self) {
        self.keycode_event_queue.clear();
        repeat_all_miniquad_input(self, self.miniquad_handler_id);
    }
}

impl miniquad::EventHandler for OrderedEventHandler {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, _ctx: &mut Context) {}

    fn key_down_event(
        &mut self,
        _ctx: &mut GraphicsContext,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        self.keycode_event_queue.push_back(InputCharacter {
            key: keycode,
            modifier: keymods,
        });
    }
}
