use serde_json::{Map, Value};
use crate::story_block::StoryType::Job;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Rect, Layout, Direction, Constraint, Alignment};
use tui::style::{Style, Color};
use tui::widgets::{Block, Text, Borders, Paragraph};

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
        let text = match item.get("text") {
            Some(t) => t.as_str().expect("Could not get Text").to_string(),
            None => "No Text".to_string()
        };

        Some(
            StoryBlock {
                stype,
                title,
                n_comments,
                link,
                text,
                score,
                author,
            })
    }
    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, chunk: Rect) {
        let story_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(70),
                ]
                    .as_ref(),
            )
            .split(chunk);


        let info = [
            Text::raw(self.text.as_str()),
            Text::raw(format!("\nLink: {}", self.link)),
            Text::raw(format!("\nPoints : {} - Comments : {} - Author: {} ",
                              self.score, self.n_comments, self.author))
        ];

        let info_p = Paragraph::new(info.iter())
            .block(Block::default().title(self.title.as_str()).borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(true);

        f.render_widget(info_p, story_chunks[0]);

    }

}
