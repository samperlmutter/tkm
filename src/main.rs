mod util;
mod system;

extern crate termion;
extern crate tui;
extern crate sysinfo;

use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget, Sparkline, Text, List};
use tui::Terminal;
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;
use tui::style::{Color, Style};
use termion::event::Key;
use sysinfo::SystemExt;

use crate::system::System;
use crate::util::event::{Event, Events};




fn main() -> Result<(), failure::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    let events = Events::new();

    let mut sys = sysinfo::System::new();
    let mut system = System::new(&mut sys, terminal.size()?.width);

    loop {
        system.update();

        terminal.draw(|mut f| {
            let main_view = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Min(0)
                    ].as_ref()
                )
                .split(f.size());
            let system_overview = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50)
                    ].as_ref()
                )
                .split(main_view[0]);
            let sparklines = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50)
                    ].as_ref()
                )
                .split(system_overview[1]);
            Sparkline::default()
                .block(
                    Block::default()
                        .title(&format!("Usage: {}% | Number of Cores: {}", system.cpu_current_usage, system.cpu_num_cores))
                        .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                        .border_style(Style::default().fg(Color::Green))
                )
                .data(&system.cpu_usage_history.as_slice())
                .style(Style::default().fg(Color::Yellow))
                .max(100)
                .render(&mut f, sparklines[0]);
            Block::default()
                .borders(Borders::ALL)
                .title("Cores")
                .render(&mut f, system_overview[0]);
            Block::default()
                .borders(Borders::ALL)
                .title("Memory")
                .render(&mut f, sparklines[1]);
            Block::default()
                .title("Processes")
                .borders(Borders::ALL)
                .render(&mut f, main_view[1]);
            // let log = system.cpu_usage_history.iter()
            //     .map(|sys|Text::raw(format!("{}", sys)));
            // List::new(log)
            //     .block(
            //         Block::default()
            //             .borders(Borders::ALL)
            //             .title("Log")
            //     )
            //     .render(&mut f, system_overview[3])
        })?;
        if let Event::Input(Key::Char('q')) = events.next()? {
            break;
        }
    }

    Ok(())
}
