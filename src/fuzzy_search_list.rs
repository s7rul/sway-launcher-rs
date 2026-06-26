use ratatui::{
    text::Line,
    widgets::{Paragraph, Widget},
};

pub struct FuzzySearchList {
    items: Vec<String>,
}

impl FuzzySearchList {
    pub fn new(items: Vec<String>) -> Self {
        Self { items }
    }
}

impl Widget for &FuzzySearchList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut lines = vec![];
        for item in &self.items {
            lines.push(Line::from(item.as_str()));
        }

        let paragraph = Paragraph::new(lines);
        paragraph.render(area, buf);
    }
}
