use ratatui::{text::Line, widgets::Widget};
use skim::{
    CaseMatching,
    fuzzy_matcher::{FuzzyMatcher, arinae::ArinaeMatcher},
};

struct ItemHolder<T> {
    name: String,
    item: T,
    rank: Option<i64>,
}

impl <T> Ord for ItemHolder<T> {
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

impl <T> PartialOrd for ItemHolder<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl <T> PartialEq for ItemHolder<T> {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.name.len() == other.name.len()
    }
}

impl <T> Eq for ItemHolder<T> {}

pub struct FuzzySearchList<T> {
    items: Vec<ItemHolder<T>>,
    matcher: ArinaeMatcher,
    select_index: usize,
    number_of_items_shown: usize,
}

impl <T: Clone> FuzzySearchList<T> {
    pub fn new(items: Vec<(String, T)>) -> Self {
        Self {
            items: items
                .iter()
                .map(|item| ItemHolder {
                    name: item.0.to_owned(),
                    item: item.1.clone(),
                    rank: Some(0),
                })
                .collect(),
            matcher: ArinaeMatcher::new(CaseMatching::Smart, true, true),
            select_index: 0,
            number_of_items_shown: items.len(),
        }
    }

    pub fn update_rankings(&mut self, key: &str) {
        self.select_index = 0;
        let mut items_not_none_count = 0;

        for item in &mut self.items {
            item.rank = self.matcher.fuzzy_match(&item.name, key);

            if item.rank.is_some() {
                items_not_none_count += 1;
            }
        }
        self.number_of_items_shown = items_not_none_count;

        self.items.sort();
    }

    pub fn move_select_up(&mut self) {
        if self.select_index < self.number_of_items_shown - 1 {
            self.select_index += 1;
        }
    }

    pub fn move_select_down(&mut self) {
        if self.select_index > 0 {
            self.select_index -= 1;
        }
    }

    pub fn get_selected(&self) -> T {
        self.items[self.items.len() - self.select_index - 1]
            .item
            .to_owned()
    }
}

impl <T> Widget for &mut FuzzySearchList<T> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        if self.select_index > area.height as usize - 1 {
            self.select_index = area.height as usize - 1;
        }

        let num_items_to_show = area.height as usize;
        for (i, item) in self
            .items
            .iter()
            .rev()
            .take(num_items_to_show)
            .filter(|item| item.rank.is_some())
            .enumerate()
        {
            let mut spans = vec![];
            if i == self.select_index {
                spans.push(" > ".into());
            } else {
                spans.push("   ".into());
            }
            spans.push(item.name.as_str().into());
            let line = Line::from(spans);
            buf.set_line(
                area.x,
                area.y + (area.height - i as u16) - 1,
                &line,
                area.width,
            );
        }
    }
}
