use std::io::Result;

use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    text::Line,
    widgets::Block,
};

use crate::fuzzy_search_list::FuzzySearchList;

pub struct App {
    should_exit: bool,
    item_list_widget: FuzzySearchList,
}

impl App {
    pub fn new(items: Vec<String>) -> Self {
        Self { should_exit: false, item_list_widget: FuzzySearchList::new(items)}
    }

    pub fn handle_event(&mut self) -> Result<()> {
        if let Some(key) = event::read()?.as_key_event() {
            match key.code {
                KeyCode::Char('q') => self.should_exit = true,
                _ => (),
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
        frame.render_widget(input_block, input_area);
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_event()?;
        }

        Ok(())
    }
}
