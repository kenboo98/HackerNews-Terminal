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
use crate::hn_api::ListType;
use crate::story_screen::StoryScreen;
use crate::tabs::TabsState;
use crate::comment_block::{Comment, CommentBlock};
#[allow(dead_code)]
mod event;
mod hn_api;
mod story_list;
mod story_screen;
mod tabs;
mod story_block;
mod comment_block;

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
                    Constraint::Percentage(93),
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
    fn select(&mut self) {
        self.screens[self.tabs.index].select()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
//    let stdout = io::stdout().into_raw_mode()?;
//    let stdout = AlternateScreen::from(stdout);
//    let backend = TermionBackend::new(stdout);
//    let mut terminal = Terminal::new(backend)?;
//    terminal.hide_cursor()?;
//
//    let mut app = App::new();
//
//    loop {
//        terminal.draw(|mut f| { app.draw(&mut f);})?;
//        match app.events.next()? {
//            Event::Input(key) => match key {
//                Key::Char('q') => {
//                    break;
//                }
//                Key::Right => app.tabs.next(),
//                Key::Left => app.tabs.previous(),
//                Key::Down => {
//                    app.next_story();
//                }
//                Key::Up => {
//                    app.previous_story();
//                },
//                Key::Char(' ') => {
//                    app.select();
//                }
//                _ => {}
//            },
//            _ => {}
//        }
//    }
    let kids:Vec<i64> = vec![
        23133642,
        23133565,
        23135659,
        23138819,
        23135184,
        23135434,
        23135467,
        23133584,
        23135636,
        23136343,
        23135565,
        23133733,
        23135450,
        23138057,
        23135596,
        23133868,
        23138642,
        23137218,
        23133836,
        23133891,
        23133653,
        23135956,
        23137800,
        23138538,
        23133620,
        23138631,
        23133894,
        23136152,
        23135125,
        23133775,
        23136039,
        23137912,
        23135111,
        23135167,
        23136074,
        23135529,
        23133803,
        23138410,
        23135194,
        23137060,
        23135962,
        23133885,
        23135475,
        23135013,
        23133648,
        23135446,
        23134556,
        23137247,
        23138452,
        23133849,
        23136283,
        23135622,
        23136103,
        23136823,
        23133649,
        23136600,
        23134197,
        23133641,
        23135666,
        23135584,
        23135545,
        23136194,
        23138715,
        23133617,
        23137124,
        23133706,
        23135718
    ];
    let coms = CommentBlock::new(kids.as_slice());
    for c in coms.comment_strings{
        print!("{}\n", c);
    }
    Ok(())
}
