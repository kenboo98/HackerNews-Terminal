use std::error::Error;
use std::io;

use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::{
    backend::TermionBackend,
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    Terminal,
    widgets::{Block, Borders, Tabs}
};
use tui::backend::Backend;

use crate::event::{Event, Events};
use crate::hn_api::StoryType;
use crate::story_screen::StoryScreen;
use crate::tabs::TabsState;
#[allow(dead_code)]
mod event;
mod hn_api;
mod story_list;
mod story_screen;
mod tabs;

struct App {
    events: Events,
    screens: Vec<StoryScreen>,
    tabs: TabsState,
}

impl App {
    fn new() -> App {
        App {
            events: Events::new(),
            screens: vec![
                StoryScreen::new(StoryType::TopStories),
                StoryScreen::new(StoryType::NewStories),
                StoryScreen::new(StoryType::BestStories),
                StoryScreen::new(StoryType::AskStories),
                StoryScreen::new(StoryType::ShowStories),
                StoryScreen::new(StoryType::JobStories),
            ],
            tabs: TabsState::new()
        }
    }
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(7),
                    Constraint::Percentage(46),
                    Constraint::Percentage(46),
                ]
                    .as_ref(),
            )
            .split(f.size());

        self.screens[self.tabs.index].draw(f, chunks[1]);
        let tabs = Tabs::default()
            .block(Block::default().borders(Borders::ALL).title("Stories"))
            .titles(self.tabs.titles.as_slice())
            .select(self.tabs.index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().fg(Color::Yellow));
        f.render_widget(tabs, chunks[0]);
    }

    fn next_story(&mut self) {
        self.screens[self.tabs.index].next()
    }

    fn previous_story(&mut self) {
        self.screens[self.tabs.index].previous()
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut app = App::new();

    loop {
        terminal.draw(|mut f| { app.draw(&mut f);})?;
        match app.events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') => {
                    break;
                }
                Key::Right => app.tabs.next(),
                Key::Left => app.tabs.previous(),
                Key::Down => {
                    app.next_story();
                }
                Key::Up => {
                    app.previous_story();
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
