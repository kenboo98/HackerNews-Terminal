use std::cmp::min;

use serde_json::{Map, Value};
use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use tui::style::Modifier;
use tui::widgets::{Block, Borders, BorderType, List, ListState, Text};

use crate::colors::*;
use crate::hn_api::{get_items, get_stories, ListType};

const INITIAL_LOADED_ITEMS: usize = 20;

pub struct StoryList {
    pub state: ListState,
    pub items: Vec<Map<String, Value>>,
    pub ids: Vec<String>,
    pub titles: Vec<String>,
    pub focused: bool,
}

impl StoryList {
    fn to_title(item: &Map<String, Value>) -> String {
        let score = match item.get("score") {
            Some(s) => s.as_i64().expect("Could not convert score to int"),
            None => 0
        };
        let author = match item.get("by") {
            Some(s) => s.as_str().expect("Could not convert author to str"),
            None => "None"
        };
        let n_comments = match item.get("descendants") {
            Some(s) => s.as_i64().expect("Could not get number of descendants"),
            None => 0
        };
        let name = match item.get("title") {
            Some(t) => t.as_str().expect("Could not get title"),
            None => ""
        };

        let title = format!("{:>4} points | {:>3} comments | {} by {} ",
                            score, n_comments, name, author);

        title.to_string()
    }
    pub fn new(story_type: &ListType) -> StoryList {
        let ids = get_stories(story_type).expect("Could not get IDs");
        let items = get_items(&ids[..INITIAL_LOADED_ITEMS]).expect("Could not get items");
        let titles = items.iter().map(|item| { StoryList::to_title(item) }).collect();
        let mut state = ListState::default();
        state.select(Some(0));
        StoryList {
            state,
            items,
            ids,
            titles,
            focused: true,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    if i < self.ids.len() - 1 {
                        let n_loads = min(self.ids.len() - i, INITIAL_LOADED_ITEMS);
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

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, chunk: Rect) {
        let mut block = Block::default()
            .title(" Stories ")
            .title_style(get_style(HNStyles::WhiteTitle))
            .borders(Borders::ALL)
            .style(get_style(HNStyles::WhiteBlock))
            .border_type(BorderType::Plain)
            .border_style(get_style(HNStyles::OrangeBorder));
        if self.focused {
            block = block.border_type(BorderType::Double);
        }

        let items = self.titles.iter().map(|i| Text::raw(i));
        let my_list = List::new(items)
            .block(block)
            .style(get_style(HNStyles::WhiteBlock))
            .highlight_style(get_style(HNStyles::WhiteBlock).modifier(Modifier::BOLD))
            .highlight_symbol(">>");
        f.render_stateful_widget(my_list, chunk, &mut self.state);
    }
}