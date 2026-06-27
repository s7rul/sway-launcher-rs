use std::io::Result;

use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Position},
    text::Line,
    widgets::Block,
};

use crate::{fuzzy_search_list::FuzzySearchList, input_box::InputBox};

pub struct App {
    should_exit: bool,
    item_list_widget: FuzzySearchList,
    input_box: InputBox,
}

impl App {
    pub fn new(items: Vec<String>) -> Self {
        Self { should_exit: false, item_list_widget: FuzzySearchList::new(items), input_box: InputBox::new()}
    }

    pub fn handle_event(&mut self) -> Result<()> {
        if let Some(key) = event::read()?.as_key_event() {
            match key.code {
                KeyCode::Esc => self.should_exit = true,
                _ => self.input_box.handle_event(key.code),
            }
        }

        Ok(())
    }

    pub fn render(&self, frame: &mut Frame) {
        let layout = Layout::vertical([Constraint::Min(3), Constraint::Length(3)]);
        let [item_area, input_area] = frame.area().layout(&layout);

        let item_block = Block::bordered().title(Line::from(" Items ").centered());
        let item_list_area = item_block.inner(item_area);
        frame.render_widget(item_block, item_area);

        frame.render_widget(&self.item_list_widget, item_list_area);

        let input_block = Block::bordered().title(Line::from(" Search ").centered());
        let input_box_area = input_block.inner(input_area);
        frame.render_widget(input_block, input_area);

        frame.render_widget(&self.input_box, input_box_area);

        let cursor_offset = self.input_box.get_cursor_index();
        frame.set_cursor_position(Position::new(input_box_area.x + cursor_offset, input_box_area.y));
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_event()?;
        }

        Ok(())
    }
}
