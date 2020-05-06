use std::io;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::{
    Terminal,
    backend::TermionBackend,
    widgets::{
        Block,
        Borders,
        List,
        Text
    },
    layout::{
        Layout,
        Constraint,
        Direction
    },
    style::{
        Style,
        Color,
        Modifier
    }
};

use serde_json::Value;

mod event;
mod story_list;
mod hn_api;
use crate::story_list::StoryList;
use crate::event::{Event, Events};
use crate::hn_api::StoryType;

use termion::event::Key;
use std::error::Error;
use std::borrow::Borrow;

fn main() -> Result<(), Box<dyn Error>> {


    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    let events = Events::new();
    let mut story = StoryList::new(StoryType::TopStories);
    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10)
                    ].as_ref()
                )
                .split(f.size());
            let items = story.titles.iter().map(|i| Text::raw(i));
            let my_list = List::new(items)
                .block(Block::default().title("HACKER NEWS")
                .title_style(Style::default().fg(Color::LightRed))
                .borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().modifier(Modifier::ITALIC))
                .highlight_symbol(">>");
            f.render_stateful_widget(my_list, chunks[1], &mut story.state);
        })?;
        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') => {
                    break;
                },
                Key::Down => {
                    story.next();
                },
                Key::Up => {
                    story.previous();
                },
                _ => {}
            }
            _ => {}
        }
    }
    Ok(())

}