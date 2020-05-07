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
        Text,
        Tabs
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
mod tabs;

use crate::tabs::TabsState;
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
    let mut tabs: TabsState<'static> = TabsState::new();
    let mut story = StoryList::new(StoryType::AskStories);
    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(7),
                        Constraint::Percentage(83),
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
                },
                Key::Right => tabs.next(),
                Key::Left => tabs.previous(),
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