mod util;
mod system;
mod rendering;

extern crate termion;
extern crate tui;
extern crate sysinfo;

use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction};
use tui::widgets::{Block, Borders, Widget, Text, List};
use tui::Terminal;
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;
use termion::event::Key;
use sysinfo::SystemExt;

use crate::system::System;
use crate::util::event::{Event, Events};
use crate::rendering::*;




fn main() -> Result<(), failure::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    terminal.hide_cursor()?;
    let events = Events::new();

    let mut sys = sysinfo::System::new();
    let mut system = System::new(&mut sys, terminal.size()?.width);

    //Defining various layouts
    let main_view = define_layout(Direction::Vertical, &[
            Constraint::Percentage(20),
            Constraint::Min(0)
        ], size);
    let system_overview = define_layout(Direction::Horizontal, &[
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ], main_view[0]);
    let sparklines = define_layout(Direction::Vertical, &[
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ], system_overview[1]);

    loop {
        system.update();

        terminal.draw(|mut f| {
            render_system_overview_layout(&mut f, sparklines[0], &system);


            // Draws borders around areas I have yet to make
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


