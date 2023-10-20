use std::collections::VecDeque;

use macroquad::color::colors::{BLACK, GRAY, LIGHTGRAY};
use macroquad::input::{is_key_pressed, is_mouse_button_pressed, mouse_position};
use macroquad::prelude::Rect;
use macroquad::prelude::{KeyCode, MouseButton, Vec2};
use macroquad::shapes::{draw_rectangle, draw_rectangle_lines};
use macroquad::text::{draw_text, measure_text};

use crate::tree::{Tree, TreeView};
use crate::ui::input_text::InputText;
use crate::ui::key_queue::InputCharacter;

pub struct Searcher {
    tag: String,
    tag_pos: Vec2,
    font_size: f32,
    rect: Rect,
    search_word: String,
    focused: bool,
    results: Vec<String>,
    nested_results: Option<Vec<TreeView>>,
    result_changed: bool,
}

impl Searcher {
    pub fn new(mut rect: Rect, font_size: f32) -> Self {
        let tag = "Search (f): ".to_string();
        let text_dimensions = measure_text(&tag, None, font_size as u16, 1.0);
        let tag_pos = Vec2::new(rect.x, rect.y);
        rect.x += text_dimensions.width;
        rect.w -= text_dimensions.width;
        rect.y -= font_size;
        Self {
            tag,
            tag_pos,
            font_size,
            rect,
            search_word: "".to_string(),
            results: Vec::new(),
            focused: false,
            nested_results: None,
            result_changed: false,
        }
    }
    pub fn get_new_result(&mut self) -> Option<&Vec<TreeView>> {
        if self.result_changed {
            self.result_changed = false;
            self.nested_results.as_ref()
        } else {
            None
        }
    }

    pub fn draw_search(&mut self, treemap: &Tree, keys: &VecDeque<InputCharacter>) {
        self.draw_search_box(keys, treemap);
        if self.focused {
            let results = &self.results;
            let line_height = 1.2 * self.font_size;
            let horizontal_pad = 0.4 * line_height;
            if results.len() > 0 {
                self.draw_candidates_dropdown(results, line_height, horizontal_pad);
            } else {
                self.draw_no_results_tooltip(line_height, horizontal_pad);
            }
        }
    }

    fn draw_search_box(&mut self, keys: &VecDeque<InputCharacter>, treemap: &Tree) {
        draw_text(
            &self.tag,
            self.tag_pos.x,
            self.tag_pos.y,
            self.font_size,
            BLACK,
        );

        let previous_search = self.search_word.clone();
        let mut input_text = InputText::new(self.rect, &mut self.search_word, keys, self.font_size);
        input_text.interact(self.focused);
        input_text.render();

        let should_search = previous_search != self.search_word;
        self.result_changed = should_search;

        if is_key_pressed(KeyCode::F) {
            self.set_focus(true);
            self.result_changed = true;
        } else if is_key_pressed(KeyCode::Enter) {
            self.set_focus(false);
        } else if is_mouse_button_pressed(MouseButton::Left)
            || is_mouse_button_pressed(MouseButton::Right)
        {
            let clicked_on_box = self.rect.contains(Vec2::from(mouse_position()));
            self.result_changed |= clicked_on_box;
            self.set_focus(clicked_on_box);
        }

        if should_search {
            self.results = treemap.search(&self.search_word, 20);
            self.results.sort_by(|a, b| a.len().cmp(&b.len()));
            if let Some(first) = self.results.first() {
                self.nested_results =
                    Some(TreeView::from_nodes(&treemap.get_nested_by_name(first)));
            } else {
                self.nested_results = Some(Vec::new());
            }
        }
    }

    fn draw_candidates_dropdown(
        &self,
        results: &Vec<String>,
        line_height: f32,
        horizontal_pad: f32,
    ) {
        let longest = results.last().unwrap();
        let dimensions = measure_text(longest, None, self.font_size as u16, 1.0);
        let w = dimensions.width + 2.0 * horizontal_pad;
        let h = (results.len() as f32 + 0.5) * line_height;
        let space = 0.0 * line_height;
        draw_rectangle(self.rect.x, self.rect.y - h - space, w, h, LIGHTGRAY);
        draw_rectangle_lines(self.rect.x, self.rect.y - h - space, w, h, 2.0, BLACK);
        draw_rectangle_lines(
            self.rect.x + horizontal_pad * 0.5,
            self.rect.y + horizontal_pad * 0.5 - h - space,
            w - horizontal_pad,
            line_height,
            2.0,
            GRAY,
        );
        for (i, result) in results.iter().enumerate() {
            draw_text(
                result,
                (self.rect.x + horizontal_pad).round(),
                (self.rect.y - h - space + (i as f32 + 1.0) * line_height).round(),
                self.font_size,
                BLACK,
            );
        }
    }

    fn draw_no_results_tooltip(&mut self, line_height: f32, horizontal_pad: f32) {
        let dimensions = measure_text(&self.search_word, None, self.font_size as u16, 1.0);
        draw_text(
            "No results",
            (self.rect.x + dimensions.width + 2.0 * horizontal_pad).round(),
            (self.tag_pos.y - line_height).round(),
            self.font_size,
            GRAY,
        );
    }

    fn set_focus(&mut self, enabled: bool) {
        if enabled {
            self.focused = true;
        } else {
            self.focused = false;
        }
    }
}
