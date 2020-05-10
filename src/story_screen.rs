use tui::{
    backend::Backend,
    Frame,
    widgets::{Borders, List},
};
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Text};

use crate::hn_api::ListType;
use crate::story_list::StoryList;
use crate::story_block::StoryBlock;

pub struct StoryScreen {
    pub story_list: StoryList,
    pub story_type: ListType,
}

impl StoryScreen {
    pub fn new(story_type: ListType) -> StoryScreen {
        StoryScreen {
            story_list: StoryList::new(&story_type),
            story_type,
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, chunk: Rect) {
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
        f.render_stateful_widget(my_list, chunk, &mut self.story_list.state);
    }
    pub fn next(&mut self) {
        self.story_list.next();
    }
    pub fn previous(&mut self) {
        self.story_list.previous();
    }
}
