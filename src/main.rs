use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders, List, Text};
use tui::layout::{Layout, Constraint, Direction};
use termion::screen::AlternateScreen;
use tui::style::{Style, Color, Modifier};

mod event;
use crate::event::{Event, Events};
use termion::event::Key;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    let events = Events::new();
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

            let items = ["Item 1", "Item 2", "Item 3"].iter().map(|i| Text::raw(*i));
            let myList = List::new(items)
                .block(Block::default().title("List").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().modifier(Modifier::ITALIC))
                .highlight_symbol(">>");
            f.render_widget(myList, chunks[1]);
        });
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