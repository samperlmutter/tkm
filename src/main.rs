#[macro_use]
mod util;
mod system;
mod render;
mod console;
mod app;
mod process;
mod command;

use std::io;
use std::io::Write;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction};
use tui::Terminal;
use termion::raw::IntoRawMode;
use termion::cursor::Goto;
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;
use termion::event::Key;

use crate::system::System;
use crate::util::*;
use crate::render::*;
use crate::app::App;


fn main() -> Result<(), failure::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = event::Events::new();

    let mut app = App::new();
    let mut system = System::new(terminal.size()?.width)?;
    let mut system_cache = System::new(terminal.size()?.width)?;

    // Sets up separate thread for polling system resources
    let (system_tx, system_rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            let system_update = system.update().unwrap();
            system_tx.send(system_update).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    loop {
        app.size = terminal.size()?;

        if let Ok(updated_system) = system_rx.try_recv() {
            system_cache = updated_system;
        }
        // Defines the upper area containing the cpu cores and graphs. Horizontally ordered
        let system_overview_constrants = vec![Constraint::Percentage(50); 2];

        // Defines areas for the cpu and memory graphs. Verically ordered
        let sparklines_constraints = vec![Constraint::Percentage(50); 2];

        // Creates as many constraints as there are cpu cores. Verically ordered
        let mut cpu_core_contraints = vec![Constraint::Length(3); system_cache.cpu_num_cores];
        cpu_core_contraints.push(Constraint::Min(0));

        // Sets the height of the upper area to be tall enough for all the cpu cores and resizes the main view to make room for the console if it's showing. Verically ordered
        let main_view_constraints = if app.console.visible {
            vec![Constraint::Length(((cpu_core_contraints.len() - 1) * 3) as u16), Constraint::Min(0), Constraint::Percentage(20), Constraint::Length(3)]
        } else {
            vec![Constraint::Length(((cpu_core_contraints.len() - 1) * 3) as u16), Constraint::Min(0), Constraint::Percentage(0), Constraint::Length(3)]
        };

        // Define layouts for the different sections of the display
        let main_view_layout = define_layout(Direction::Vertical, &main_view_constraints, terminal.size()?);
        let system_overview_layout = define_layout(Direction::Horizontal, &system_overview_constrants, main_view_layout[0]);
        let sparklines_layout = define_layout(Direction::Vertical, &sparklines_constraints, system_overview_layout[1]);
        let cpu_cores_layout = define_layout(Direction::Vertical, &cpu_core_contraints, system_overview_layout[0]);

        terminal.draw(|mut f| {
            render_sparklines_layout(&mut f, &sparklines_layout, &system_cache);
            render_cpu_cores_layout(&mut f, &cpu_cores_layout, &system_cache);
            render_processes_layout(&mut f, main_view_layout[1], &system_cache, &app);
            render_console_layout(&mut f, main_view_layout[2], &app.console);
            render_input_layout(&mut f, main_view_layout[3], &app.console.input);
        })?;

        // Positions cursor after user input
        write!(
            terminal.backend_mut(),
            "{}",
            Goto(2 + app.console.input.len() as u16, app.size.height - 1)
        )?;

        terminal.show_cursor()?;
        if let event::Event::Input(input) = events.next()? {
            match input {

                // Quit the program
                Key::Ctrl('c') => {
                    break;
                }

                // Toggle showing the debugging log
                Key::Char('/') => {
                    app.processes_sort_by = SortBy::CPU;
                    app.console.toggle_visibility();
                }

                // Attempt to process current input as command
                Key::Char('\n') => {
                    app.process_command();
                }

                // Capture text input into the console
                Key::Char(c) => {
                    app.console.append_input(c);
                }

                Key::Backspace => {
                    app.console.backspace();
                }

                _ => {}
            }
        }
        terminal.hide_cursor()?;
    }

    Ok(())
}
