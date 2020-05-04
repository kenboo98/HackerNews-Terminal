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

mod event;
mod hn_api;
use crate::event::{Event, Events};
use crate::hn_api::best_stories;

use termion::event::Key;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {


    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    let events = Events::new();
    let best_stories = best_stories().unwrap();
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
            let items = best_stories.iter().map(|i| Text::raw(i));
            let my_list = List::new(items)
                .block(Block::default().title("List").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().modifier(Modifier::ITALIC))
                .highlight_symbol(">>");
            f.render_widget(my_list, chunks[1]);
        })?;
        match events.next()? {
            Event::Input(key) => {
                if key == Key::Char('q') {
                    break;
                }
            }
            _ => {}
        }
    }
    Ok(())

}