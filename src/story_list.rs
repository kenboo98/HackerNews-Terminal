use serde_json::{Map, Value};
use tui::widgets::ListState;

use crate::hn_api::{get_items, get_stories, StoryType};

const INITIAL_LOADED_ITEMS: usize = 20;

pub struct StoryList {
    pub state: ListState,
    pub items: Vec<Map<String, Value>>,
    pub ids: Vec<String>,
    pub titles: Vec<String>,
}

impl StoryList {
    pub fn new(story_type: &StoryType) -> StoryList {
        let ids = get_stories(story_type).expect("Could not get IDs");
        let items = get_items(&ids[..INITIAL_LOADED_ITEMS]).expect("Could not get items");
        let titles = items.iter().map(|item| { item["title"].to_string() }).collect();
        StoryList {
            state: ListState::default(),
            items,
            ids,
            titles,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    if i < self.ids.len() - 1 {
                        self.items.append(get_items(&self.ids[i + 1..i + INITIAL_LOADED_ITEMS])
                            .expect("Could not get new item").as_mut());
                        for item in &self.items[i + 1..i + INITIAL_LOADED_ITEMS] {
                            self.titles.push(item["title"].to_string())
                        }
                        i + 1
                    } else {
                        0
                    }
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