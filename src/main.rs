use std::io;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, Tabs, Text},
    Terminal,
};

use serde_json::Value;

mod event;
mod hn_api;
mod story_list;
mod story_screen;
mod tabs;

use crate::event::{Event, Events};
use crate::hn_api::StoryType;
use crate::story_screen::StoryScreen;
use crate::tabs::TabsState;

use std::borrow::Borrow;
use std::error::Error;
use termion::event::Key;

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    let events = Events::new();
    let mut tabs: TabsState<'static> = TabsState::new();
    let mut story = StoryScreen::new(StoryType::AskStories);
    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(7),
                        Constraint::Percentage(83),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            story.draw(&mut f, chunks[1]);
            let tabs = Tabs::default()
                .block(Block::default().borders(Borders::ALL).title("Stories"))
                .titles(tabs.titles.as_slice())
                .select(tabs.index)
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(Style::default().fg(Color::Yellow));
            f.render_widget(tabs, chunks[0]);
        })?;
        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') => {
                    break;
                }
                Key::Right => tabs.next(),
                Key::Left => tabs.previous(),
                Key::Down => {
                    story.next();
                }
                Key::Up => {
                    story.previous();
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
