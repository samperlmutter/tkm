mod util;
mod system;
mod render;
mod log;
mod app;
mod process;

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
    terminal.hide_cursor()?;
    let events = Events::new();

    let mut log = Log::new();
    let mut system = System::new(sysinfo::System::new(), terminal.size()?.width)?;
    let mut app = App::new();


    loop {
        system.update()?;

        // Resizes the main view to make room for the debug log if it's showing
        let main_view_constraints = if log.show_log {
            vec![Constraint::Percentage(25), Constraint::Percentage(45), Constraint::Min(0)]
        } else {
            vec![Constraint::Percentage(25), Constraint::Percentage(75), Constraint::Min(0)]
        };
        let system_overview_constrants = vec![Constraint::Percentage(50); 2];
        let sparklines_constraints = vec![Constraint::Percentage(50); 2];
        // Creates as many constraints as there are cpu cores
        let mut cpu_core_contraints = vec![Constraint::Length(3); system.cpu_num_cores];
        cpu_core_contraints.push(Constraint::Min(0));

        let main_view_layout = define_layout(Direction::Vertical, &main_view_constraints, terminal.size()?);
        let system_overview_layout = define_layout(Direction::Horizontal, &system_overview_constrants, main_view_layout[0]);
        let sparklines_layout = define_layout(Direction::Vertical, &sparklines_constraints, system_overview_layout[1]);
        let cpu_cores_layout = define_layout(Direction::Vertical, &cpu_core_contraints, system_overview_layout[0]);

        terminal.draw(|mut f| {
            render_sparklines_layout(&mut f, &sparklines_layout, &system);
            render_cpu_cores_layout(&mut f, &cpu_cores_layout, &system);
            render_processes_layout(&mut f, &main_view_layout, &system);
            render_log(&log, &mut f, main_view_layout[2]);
        })?;

        if let Event::Input(input) = events.next()? {
            match input {
                // Quit the program
                Key::Char('q') => {
                    break;
                }
                // Toggle showing the debugging log
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
