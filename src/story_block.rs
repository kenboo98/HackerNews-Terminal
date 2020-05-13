use std::collections::HashSet;

use ammonia::Builder;
use serde_json::{Map, Value};
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::widgets::{Block, Borders, BorderType, Paragraph, Text};

use crate::colors::{get_style, HNStyles};

pub enum StoryType {
    Job,
    Story,
    Poll,
}

pub struct StoryBlock {
    pub stype: StoryType,
    pub title: String,
    pub n_comments: i64,
    pub link: String,
    pub text: String,
    pub score: i64,
    pub author: String,
    pub focused: bool,
    scroll: u16,
}

impl StoryBlock {
    pub fn new(item: &Map<String, Value>) -> Option<StoryBlock> {
        let stype = match item["type"].as_str() {
            Some(t) => {
                match t {
                    "job" => StoryType::Job,
                    "story" => StoryType::Story,
                    "poll" => StoryType::Poll,
                    _ => { return None; }
                }
            }
            None => {
                return None;
            }
        };

        let score = match item.get("score") {
            Some(s) => s.as_i64().expect("Could not convert score to int"),
            None => 0
        };
        let author = match item.get("by") {
            Some(s) => s.as_str().expect("Could not convert author to str").to_string(),
            None => "None".to_string()
        };
        let n_comments = match item.get("descendants") {
            Some(s) => s.as_i64().expect("Could not get number of descendants"),
            None => 0
        };
        let title = match item.get("title") {
            Some(t) => t.as_str().expect("Could not get title").to_string(),
            None => "".to_string()
        };
        let link = match item.get("url") {
            Some(t) => t.as_str().expect("Could not get link").to_string(),
            None => "No Link".to_string()
        };
        let text_raw = match item.get("text") {
            Some(t) => t.as_str().expect("Could not get Text").to_string(),
            None => "No Text".to_string()
        };

        // Remove HTML tags using ammonia
        let text = Builder::new()
            .tags(HashSet::new())
            .clean(text_raw.as_str())
            .to_string();

        Some(
            StoryBlock {
                stype,
                title,
                n_comments,
                link,
                text,
                score,
                author,
                focused: false,
                scroll: 0,
            })
    }
    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, chunk: Rect) {
        let info = [
            Text::raw(format!("Link: {}\n", self.link)),
            Text::raw(format!("Points : {} - Comments : {} - Author: {} \n",
                              self.score, self.n_comments, self.author)),
            Text::raw(self.text.as_str()),
        ];

        let mut block = Block::default()
            .title("Info")
            .title_style(get_style(HNStyles::WhiteTitle))
            .borders(Borders::ALL)
            .style(get_style(HNStyles::WhiteBlock))
            .border_style(get_style(HNStyles::OrangeBorder))
            .border_type(BorderType::Plain);
        if self.focused {
            block = block.border_type(BorderType::Double);
        }

        let info_p = Paragraph::new(info.iter())
            .block(block)
            .style(get_style(HNStyles::WhiteBlock))
            .alignment(Alignment::Left)
            .wrap(true)
            .scroll(self.scroll);

        f.render_widget(info_p, chunk);
    }

    pub fn scroll_down(&mut self) {
        self.scroll += 1
    }
    pub fn scroll_up(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1
        }
    }
}
