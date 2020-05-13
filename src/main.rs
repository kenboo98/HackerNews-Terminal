use std::error::Error;
use std::io;

use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::{
    backend::TermionBackend,
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    Terminal,
    widgets::{Block, Borders, Tabs},
};
use tui::backend::Backend;


#[allow(dead_code)]
mod event;
mod hn_api;
mod story_list;
mod story_screen;
mod tabs;
mod story_block;
mod comment_block;
mod colors;

use crate::event::{Event, Events};
use crate::hn_api::ListType;
use crate::story_screen::StoryScreen;
use crate::tabs::TabsState;
use crate::colors::{HNStyles, get_style, HN_ORANGE, HN_BACKGROUND};

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
                StoryScreen::new(ListType::TopStories),
                StoryScreen::new(ListType::NewStories),
                StoryScreen::new(ListType::BestStories),
                StoryScreen::new(ListType::AskStories),
                StoryScreen::new(ListType::ShowStories),
                StoryScreen::new(ListType::JobStories),
            ],
            tabs: TabsState::new(),
        }
    }
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Min(3),
                    Constraint::Percentage(100),
                ]
                    .as_ref(),
            )
            .split(f.size());

        let main_block = Block::default().style(Style::new().bg(HN_BACKGROUND));
        f.render_widget(main_block, f.size());
        self.screens[self.tabs.index].draw(f, chunks[1]);
        let tabs = Tabs::default()
            .block(Block::default().borders(Borders::ALL)
                .title("Hacker News").title_style(get_style(HNStyles::OrangeTitle).modifier(Modifier::BOLD))
                .border_style(Style::default().bg(HN_ORANGE).fg(HN_ORANGE))
                .style(get_style(HNStyles::OrangeBlock)))
            .titles(self.tabs.titles.as_slice())
            .select(self.tabs.index)
            .style(get_style(HNStyles::OrangeBlock))
            .highlight_style(Style::default().fg(Color::Black).bg(HN_ORANGE));
        f.render_widget(tabs, chunks[0]);
    }

    fn down(&mut self) {
        self.screens[self.tabs.index].down()
    }

    fn up(&mut self) {
        self.screens[self.tabs.index].up()
    }
    fn select(&mut self) {
        self.screens[self.tabs.index].select()
    }
    fn focus(&mut self) { self.screens[self.tabs.index].focus() }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut app = App::new();

    loop {
        terminal.draw(|mut f| { app.draw(&mut f); })?;
        match app.events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') => {
                    break;
                }
                Key::Right => app.tabs.next(),
                Key::Left => app.tabs.previous(),
                Key::Down => {
                    app.down();
                }
                Key::Up => {
                    app.up();
                }
                Key::Char('\n') => {
                    app.select();
                }
                Key::Char('\x09') => {
                    app.focus();
                }

                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}
