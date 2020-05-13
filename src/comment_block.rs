use crate::hn_api::get_comments;
use crate::colors::{HNStyles, get_style};

use ammonia::Builder;
use std::collections::HashSet;
use tui::Frame;
use tui::layout::{Rect, Alignment};
use tui::backend::Backend;
use tui::widgets::{Paragraph, Block, Borders, Text, BorderType};
use serde_json::{Map, Value};
use tui::style::Style;
use std::cmp::min;

const MAX_DEPTH: u16 = 10;
pub struct Comment {
    pub text: String,
    pub replies: Option<Vec<Comment>>,
}

pub struct CommentBlock {
    pub comments: Vec<Comment>,
    pub comment_strings: Vec<String>,
    pub focused: bool,
    scroll: u16,
}

impl CommentBlock {
    fn helper(c: &Comment, depth: u16, builder: &Builder) -> Vec<String> {
        let prefix = "--".repeat(min(depth, MAX_DEPTH) as usize);
        let mut result = vec![format!("{}> {}\n", prefix, builder.clean(c.text.as_str()))];
        match &c.replies {
            Some(replies) => {
                for reply in replies {
                    result.append(&mut CommentBlock::helper(reply, depth + 1, builder));
                }
            }
            None => {}
        }
        return result;
    }
    pub fn new(item: &Map<String, Value>) -> Option<CommentBlock> {
        let comment_ids = match item.get("kids") {
            Some(kids) => kids.as_array().unwrap(),
            None => return { None }
        };

        let comment_ids: Vec<i64> = comment_ids.into_iter().map(|id| id.as_i64().unwrap()).collect();

        let comments = match get_comments(comment_ids.as_slice()) {
            Ok(c) => c,
            Err(_) => {
                return None;
            }
        };
        let mut builder = Builder::new();
        let tag_cleaner = builder.tags(HashSet::new());
        let mut comment_strings = Vec::new();
        for c in &comments {
            comment_strings.append(&mut CommentBlock::helper(c, 0, &tag_cleaner));
        };

        Some(CommentBlock {
            comments,
            comment_strings,
            focused: false,
            scroll: 0,
        })
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, chunk: Rect) {
        let comment_text: Vec<Text> = self.comment_strings.iter().map(|c| Text::raw(c)).collect();

        let mut block = Block::default()
            .title("Comments")
            .title_style(get_style(HNStyles::WhiteTitle))
            .borders(Borders::ALL)
            .style(get_style(HNStyles::WhiteBlock))
            .border_style(get_style(HNStyles::OrangeBorder))
            .border_type(BorderType::Plain);

        if self.focused {
            block = block.border_type(BorderType::Double);
        }

        let paragraph = Paragraph::new(comment_text.iter())
            .block(block)
            .style(get_style(HNStyles::WhiteBlock))
            .alignment(Alignment::Left)
            .wrap(true)
            .scroll(self.scroll);

        f.render_widget(paragraph, chunk);
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