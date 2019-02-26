#[macro_use]
mod util;
mod system;
mod render;
mod console;
mod app;
mod process;

use std::io;
use std::io::Write;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction};
use tui::Terminal;
use termion::raw::IntoRawMode;
use termion::cursor::Goto;
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;
use termion::event::Key;
use sysinfo::SystemExt;

use crate::system::System;
use crate::util::*;
use crate::render::*;
use crate::console::*;
use crate::app::*;
use crate::process::Process;


fn main() -> Result<(), failure::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = event::Events::new();

    let mut console = Console::new();
    let mut system = System::new(sysinfo::System::new(), terminal.size()?.width)?;
    let mut app = App::new();

    loop {
        system.update()?;
        app.size = terminal.size()?;

        // Resizes the main view to make room for the console if it's showing
        let main_view_constraints = if console.visible {
            vec![Constraint::Percentage(25), Constraint::Percentage(45), Constraint::Min(0), Constraint::Length(2)]
        } else {
            vec![Constraint::Percentage(25), Constraint::Min(0), Constraint::Length(3)]
        };
        let system_overview_constrants = vec![Constraint::Percentage(50); 2];
        let sparklines_constraints = vec![Constraint::Percentage(50); 2];

        // Creates as many constraints as there are cpu cores
        let mut cpu_core_contraints = vec![Constraint::Length(3); system.cpu_num_cores];
        cpu_core_contraints.push(Constraint::Min(0));

        // Define layouts for the different sections of the display
        let main_view_layout = define_layout(Direction::Vertical, &main_view_constraints, terminal.size()?);
        let system_overview_layout = define_layout(Direction::Horizontal, &system_overview_constrants, main_view_layout[0]);
        let sparklines_layout = define_layout(Direction::Vertical, &sparklines_constraints, system_overview_layout[1]);
        let cpu_cores_layout = define_layout(Direction::Vertical, &cpu_core_contraints, system_overview_layout[0]);

        terminal.hide_cursor()?;
        terminal.draw(|mut f| {
            render_sparklines_layout(&mut f, &sparklines_layout, &system);
            render_cpu_cores_layout(&mut f, &cpu_cores_layout, &system);
            // render_console(&mut f, main_view_layout[2], &console);
            render_input_layout(&mut f, main_view_layout[2], &console.buffer);

            // Controls how the processes list is sorted
            match app.processes_sort_by {
                SortBy::PID => {
                    render_processes_layout(&mut f, &main_view_layout, &sort_processes!(system.processes, Process.pid, app.processes_sort_direction));
                }
                SortBy::Name => {
                    render_processes_layout(&mut f, &main_view_layout, &sort_processes!(system.processes, Process.name, app.processes_sort_direction));
                }
                SortBy::CPU => {
                    render_processes_layout(&mut f, &main_view_layout, &sort_processes!(system.processes, Process.cpu, app.processes_sort_direction));
                }
                SortBy::Memory => {
                    render_processes_layout(&mut f, &main_view_layout, &sort_processes!(system.processes, Process.mem, app.processes_sort_direction));
                }
            }
        })?;

        terminal.show_cursor()?;
        write!(
            terminal.backend_mut(),
            "{}",
            Goto(1 + console.buffer.len() as u16, app.size.height)
        )?;

        if let event::Event::Input(input) = events.next()? {
            match input {
                // Quit the program
                Key::Char('q') => {
                    break;
                }
                // Toggle showing the debugging log
                // Key::Char('/') => {
                //     console.toggle_visibility();
                // }
                // Sort processes by CPU usage
                Key::Char('c') => {
                    app.processes_sort_by = SortBy::CPU;
                    app.toggle_sort_direction();
                }
                // Sort processes by memory usage
                Key::Char('m') => {
                    app.processes_sort_by = SortBy::Memory;
                    app.toggle_sort_direction();
                }
                // Sort processes by PID
                Key::Char('p') => {
                    app.processes_sort_by = SortBy::PID;
                    app.toggle_sort_direction();
                }
                // Enter input into the console
                Key::Char(c) => {
                    console.buffer.push(c);
                }
                _ => {}
            }
        }
    }

    Ok(())
}
