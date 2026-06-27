use crossterm::event::KeyCode;
use ratatui::{style::Style, widgets::Widget};

pub struct InputBox {
    buffer: Vec<char>,
    index: usize,
}

impl InputBox {
    pub fn new() -> Self {
        Self {
            buffer: vec![],
            index: 0,
        }
    }

    pub fn handle_event(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Left => {
                if self.index > 0 {
                    self.index -= 1;
                }
            }
            KeyCode::Right => {
                if self.index < self.buffer.len() {
                    self.index += 1;
                }
            }
            KeyCode::Backspace => {
                if self.index > 0 {
                    self.index -= 1;
                    self.buffer.remove(self.index);
                }
            }
            KeyCode::Char(c) => {
                self.buffer.insert(self.index, c);
                self.index += 1;
            }
            _ => (),
        }
    }

    pub fn get_cursor_index(&self) -> u16 {
        self.index as u16
    }

    pub fn get_current_input(&self) -> String {
        self.buffer.iter().collect()
    }
}

impl Widget for &InputBox {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        buf.set_string(
            area.x,
            area.y,
            self.buffer.iter().collect::<String>(),
            Style::default(),
        );
    }
}
