use serde_json::{Map, Value};
use tui::widgets::ListState;

use crate::hn_api::{get_items, get_stories, StoryType};
use std::cmp::min;

const INITIAL_LOADED_ITEMS: usize = 20;

pub struct StoryList {
    pub state: ListState,
    pub items: Vec<Map<String, Value>>,
    pub ids: Vec<String>,
    pub titles: Vec<String>,
}

impl StoryList {

    fn to_title(item: &Map<String, Value>) -> String {
        let score = match item.get("score") {
            Some(s) => s.as_i64().expect("Could not convert score to str"),
            None => 0
        };
        let author = match item.get("by") {
            Some(s) => s.as_str().expect("Could not convert author to str"),
            None => "None"
        };
        let n_comments = match item.get("kids"){
            Some(arr) => arr.as_array().expect("kids is not an array").len(),
            None => 0
        };
        let name = match item.get("title") {
            Some(t) => t.as_str().expect("Could not get title"),
            None => ""
        };

        let title = format!("{:^5} points | {:^3} comments | {} by {} ",
                            score, n_comments, name, author);

        title.to_string()

    }
    pub fn new(story_type: &StoryType) -> StoryList {
        let ids = get_stories(story_type).expect("Could not get IDs");
        let items = get_items(&ids[..INITIAL_LOADED_ITEMS]).expect("Could not get items");
        let titles = items.iter().map(|item| { StoryList::to_title(item) }).collect();
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
                        let n_loads = min(self.ids.len()-i,INITIAL_LOADED_ITEMS);
                        self.items.append(get_items(&self.ids[i + 1..i + n_loads])
                            .expect("Could not get new item").as_mut());
                        for item in &self.items[i + 1..i + n_loads] {
                            self.titles.push(StoryList::to_title(item))
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