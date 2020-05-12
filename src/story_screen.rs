use tui::{
    backend::Backend,
    Frame,
    widgets::{Borders, List},
};
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Text};

use crate::hn_api::ListType;
use crate::story_list::StoryList;
use crate::story_block::StoryBlock;
use std::borrow::Borrow;
use crate::comment_block::CommentBlock;

// Struct to select each block to scroll
enum Focus {
    List,
    Info,
    Comments,
}
pub struct StoryScreen {
    pub story_list: StoryList,
    pub story_type: ListType,
    pub story_block: Option<StoryBlock>,
    pub comment_block: Option<CommentBlock>,
    focused: Focus,
}

impl StoryScreen {
    pub fn new(story_type: ListType) -> StoryScreen {
        StoryScreen {
            story_list: StoryList::new(&story_type),
            story_type,
            story_block: None,
            comment_block: None,
            focused: Focus::List,
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, chunk: Rect) {
        let story_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(15),
                    Constraint::Percentage(55)
                ]
                    .as_ref(),
            )
            .split(chunk);

        self.story_list.draw(f, story_chunks[0]);

        match self.story_block.as_mut() {
            Some(s) => s.draw(f, story_chunks[1]),
            None => {}
        }

        match self.comment_block.as_mut() {
            Some(c) => c.draw(f, story_chunks[2]),
            None => {},
        }

    }
    pub fn down(&mut self) {
        match self.focused {
            Focus::List => {self.story_list.next()},
            Focus::Info => {
                match self.story_block.as_mut() {
                    Some(s) => s.scroll_down(),
                    None => {}
                }
            },
            Focus::Comments => {
                match self.comment_block.as_mut() {
                    Some(c) => c.scroll_down(),
                    None => {}
                }
            }
        }
    }
    pub fn up(&mut self) {
        match self.focused {
            Focus::List => {self.story_list.previous()},
            Focus::Info => {
                match self.story_block.as_mut() {
                    Some(s) => s.scroll_up(),
                    None => {}
                }
            },
            Focus::Comments => {
                match self.comment_block.as_mut() {
                    Some(c) => c.scroll_up(),
                    None => {}
                }
            }
        };
    }

    pub fn select(&mut self) {
        let selected = self.story_list.state.selected().unwrap();
        if let Some(s) = StoryBlock::new(self.story_list.items[selected].borrow()) {
            self.story_block.replace(s);
        };

        if let Some(c) = CommentBlock::new(self.story_list.items[selected].borrow()) {
            self.comment_block.replace(c);
        };
    }

    pub fn focus(&mut self) {
        match self.focused {
            Focus::List => {
                if let Some(s) = self.story_block.as_mut(){
                    self.focused = Focus::Info;
                    self.story_list.focused = false;
                    s.focused = true;
                }
            },
            Focus::Info => {
                if let Some(c) = self.comment_block.as_mut(){
                    self.focused = Focus::Comments;
                    self.story_block.as_mut().unwrap().focused = false;
                    c.focused = true;
                }
            },
            Focus::Comments => {
                self.focused = Focus::List;
                self.comment_block.as_mut().unwrap().focused = false;
                self.story_list.focused = true;
            }
        }
    }

}
