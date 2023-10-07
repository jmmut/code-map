use crate::tree::{Tree, TreeView};
use macroquad::color::colors::{BLACK, GRAY, LIGHTGRAY};
use macroquad::hash;
use macroquad::input::{is_key_pressed, is_mouse_button_pressed, mouse_position};
use macroquad::prelude::Rect;
use macroquad::prelude::{KeyCode, MouseButton, Vec2};
use macroquad::shapes::{draw_rectangle, draw_rectangle_lines};
use macroquad::text::{draw_text, measure_text};
use macroquad::ui::root_ui;
use macroquad::ui::widgets::InputText;

pub struct Searcher {
    ui_id: u64,
    tag: String,
    tag_pos: Vec2,
    font_size: f32,
    rect: Rect,
    search_word: String,
    focused: bool,
    result: Option<String>,
    results: Vec<String>,
    nested_results: Option<Vec<TreeView>>,
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
            ui_id: hash!(rect.x as u64, rect.y as u64, rect.w as u64, rect.h as u64),
            tag,
            tag_pos,
            font_size,
            rect,
            search_word: "".to_string(),
            results: Vec::new(),
            focused: false,
            result: None,
            nested_results: None,
        }
    }
    pub fn get_result(&self) -> Option<&Vec<TreeView>> {
        if self.focused {
            self.nested_results.as_ref()
        } else {
            None
        }
    }

    pub fn draw_search(&mut self, treemap: &Tree) {
        let previous_search = self.search_word.clone();
        if let Some(search_word) = self.draw_search_box() {
            if previous_search != search_word {
                self.results = treemap.search(&search_word, 20);
                self.results.sort_by(|a, b| a.len().cmp(&b.len()));
                if let Some(first) = self.results.first() {
                    self.nested_results =
                        Some(TreeView::from_nodes(&treemap.get_nested_by_name(first)));
                }
            }
            let results = &self.results;
            let line_height = 1.2 * self.font_size;
            let horizontal_pad = 0.4 * line_height;
            if results.len() > 0 {
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
                self.result = results.first().cloned();
            } else {
                let dimensions = measure_text(&self.search_word, None, self.font_size as u16, 1.0);
                draw_text(
                    "No results",
                    (self.rect.x + dimensions.width + 2.0 * horizontal_pad).round(),
                    (self.tag_pos.y - line_height).round(),
                    self.font_size,
                    GRAY,
                );
                self.result = None
            }
        } else {
            self.result = None;
        }
    }

    fn draw_search_box(&mut self) -> Option<String> {
        draw_text(
            &self.tag,
            self.tag_pos.x,
            self.tag_pos.y,
            self.font_size,
            BLACK,
        );

        InputText::new(self.ui_id)
            .position(self.rect.point())
            .size(self.rect.size())
            .ui(&mut root_ui(), &mut self.search_word);

        if is_key_pressed(KeyCode::F) {
            self.set_focus(true);
        } else if is_key_pressed(KeyCode::Enter) {
            self.set_focus(false);
        } else if is_mouse_button_pressed(MouseButton::Left) {
            self.set_focus(self.rect.contains(Vec2::from(mouse_position())));
        }
        if self.focused && !self.search_word.is_empty() {
            Some(self.search_word.clone())
        } else {
            None
        }
    }
    fn set_focus(&mut self, enabled: bool) {
        if enabled {
            self.focused = true;
            root_ui().set_input_focus(self.ui_id);
        } else {
            self.focused = false;
            root_ui().clear_input_focus();
        }
    }
}
