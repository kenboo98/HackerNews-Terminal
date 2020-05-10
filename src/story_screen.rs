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

pub struct StoryScreen {
    pub story_list: StoryList,
    pub story_type: ListType,
    pub story_block: Option<StoryBlock>
}

impl StoryScreen {
    pub fn new(story_type: ListType) -> StoryScreen {
        StoryScreen {
            story_list: StoryList::new(&story_type),
            story_type,
            story_block: None
        }
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
        let items = self.story_list.titles.iter().map(|i| Text::raw(i));
        let my_list = List::new(items)
            .block(
                Block::default()
                    .title("HACKER NEWS")
                    .title_style(Style::default().fg(Color::LightRed))
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().modifier(Modifier::BOLD))
            .highlight_symbol(">>");
        f.render_stateful_widget(my_list, story_chunks[0], &mut self.story_list.state);

        match self.story_block.as_mut() {
            Some(s) => s.draw(f, story_chunks[1]),
            None => {}
        }

    }
    pub fn next(&mut self) {
        self.story_list.next();
    }
    pub fn previous(&mut self) {
        self.story_list.previous();
    }

    pub fn select(&mut self) {
        let selected = self.story_list.state.selected().unwrap();
        self.story_block.replace(StoryBlock::new(self.story_list.items[selected].borrow()).unwrap());
    }

}
