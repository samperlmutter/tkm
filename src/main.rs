#[allow(dead_code)]
mod util;

extern crate termion;
extern crate tui;
extern crate sys_info;
extern crate spork;

use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget, Sparkline};
use tui::Terminal;
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;
use tui::style::{Color, Style};
use termion::event::Key;
use spork::{Spork};

use crate::util::event::{Event, Events};

fn main() -> Result<(), failure::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    terminal.hide_cursor()?;
    let v = vec![];
    let events = Events::new();
    let spork = match Spork::new() {
        Ok(s) => s,
        Err(e) => panic!("spork error: {:?}", e)
    };
    let mut cpu = spork.clock_speed();

    loop {
        // cpu = sys_info::cpu_speed()? as u64;
        // println!("{}", sys_info::cpu_speed()?);
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(7),
                        Constraint::Min(0)
                    ].as_ref()
                )
                .split(f.size());
            Sparkline::default()
                .block(
                    Block::default()
                        .title(&format!("{}", cpu))
                        .borders(Borders::LEFT | Borders::RIGHT)
                )
                .data(&v)
                .style(Style::default().fg(Color::Yellow))
                .render(&mut f, chunks[0]);
                
        })?;
        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => break,
                _ => {}
            }
            Event::Tick => {
                cpu = spork.clock_speed();
            }
        }
    }

    Ok(())
}
