use ratatui::{
    text::Line,
    widgets::{Paragraph, Widget},
};
use skim::{
    CaseMatching,
    fuzzy_matcher::{FuzzyMatcher, arinae::ArinaeMatcher},
};

struct ItemHolder {
    name: String,
    rank: i64,
}

impl Ord for ItemHolder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.rank.cmp(&other.rank) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => {
                if self.name.len() == other.name.len() {
                    std::cmp::Ordering::Equal
                } else if self.name.len() < other.name.len() {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            }
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for ItemHolder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ItemHolder {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.name.len() == other.name.len()
    }
}

impl Eq for ItemHolder {}

pub struct FuzzySearchList {
    items: Vec<ItemHolder>,
    matcher: ArinaeMatcher,
}

impl FuzzySearchList {
    pub fn new(items: Vec<String>) -> Self {
        Self {
            items: items
                .iter()
                .map(|name| ItemHolder {
                    name: name.to_owned(),
                    rank: 0,
                })
                .collect(),
            matcher: ArinaeMatcher::new(CaseMatching::Smart, true, true),
        }
    }

    pub fn update_rankings(&mut self, key: &str) {
        for item in &mut self.items {
            match self.matcher.fuzzy_match(&item.name, key) {
                Some(rank) => item.rank = rank,
                None => item.rank = -1,
            }
        }

        self.items.sort();
    }
}

impl Widget for &FuzzySearchList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let num_items_to_show = area.height as usize;
        let mut lines = vec![];
        for item in self.items.iter().rev().take(num_items_to_show).rev() {
            lines.push(Line::from(vec![
                format!("{} - ", item.rank).into(),
                item.name.as_str().into(),
            ]));
        }

        let paragraph = Paragraph::new(lines);
        paragraph.render(area, buf);
    }
}
