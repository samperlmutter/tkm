mod util;
mod system;
mod render;
mod log;
mod app;

extern crate termion;
extern crate tui;
extern crate sysinfo;

use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction};
use tui::Terminal;
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;
use termion::event::Key;
use sysinfo::SystemExt;

use crate::system::System;
use crate::util::event::{Event, Events};
use crate::render::*;
use crate::log::*;
use crate::app::*;


fn main() -> Result<(), failure::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    terminal.hide_cursor()?;
    let events = Events::new();

    let mut log = Log::new();
    let mut system = System::new(sysinfo::System::new(), terminal.size()?.width)?;
    let mut app = App::new();

    //Defining various layouts
    let main_view_layout = define_layout(Direction::Vertical, &[
            Constraint::Percentage(25),
            Constraint::Min(0)
        ], size);
    let system_overview_layout = define_layout(Direction::Horizontal, &[
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ], main_view_layout[0]);
    let sparklines_layout = define_layout(Direction::Vertical, &[
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ], system_overview_layout[1]);
    let log_layout = define_layout(Direction::Vertical, &[
            Constraint::Percentage(70),
            Constraint::Percentage(30)
        ], size);
    let cpu_cores_layout = define_layout(Direction::Vertical, &[
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0)
        ], system_overview_layout[0]);

    loop {
        system.update()?;

        terminal.draw(|mut f| {
            render_sparklines_layout(&mut f, &sparklines_layout, &system);
            render_cpu_cores_layout(&mut f, &cpu_cores_layout, &system);
            render_processes_layout(&mut f, &main_view_layout, &system);

            log.render(&mut f, log_layout[1]);
        })?;

        if let Event::Input(input) = events.next()? {
            match input {
                Key::Char('q') => {
                    break;
                }
                Key::Char('l') => {
                    log.toggle_log();
                }
                Key::Up => {
                    app.move_cursor_up();
                }
                Key::Down => {
                    app.move_cursor_down();
                }
                _ => {}
            }
        }
    }

    Ok(())
}
