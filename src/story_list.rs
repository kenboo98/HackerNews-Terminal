use tui::widgets::ListState;
use serde_json::{Value, Number, Map};

use crate::hn_api::{get_stories, items, StoryType};

pub struct StoryList {
    pub state: ListState,
    pub items: Vec<Map<String, Value>>,
    pub ids: Vec<String>,
    pub titles: Vec<String>1
}

impl StoryList {

    pub fn new(story_type: StoryType, max_size: i32) -> StoryList {
        let ids = get_stories(story_type).expect("Could not get IDs");
        let items = items(&ids[..50]).expect("Could not get items");
        let titles = items.iter().map(|item|{item["title"].to_string()}).collect();
        StoryList {
            state: ListState::default(),
            items,
            ids,
            titles
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}