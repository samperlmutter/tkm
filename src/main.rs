#[allow(dead_code)]
mod util;

extern crate termion;
extern crate tui;
// extern crate sys_info;
// extern crate spork;
extern crate sysinfo;

use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget, Sparkline, List, Text};
use tui::Terminal;
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;
use tui::style::{Color, Style};
use termion::event::Key;
// use spork::{Spork, StatType, SporkError};
use sysinfo::{SystemExt, ProcessorExt, System};

use crate::util::event::{Event, Events};

struct CPU {
    usage_history: Vec<u64>,
    current_usage: u64,
    num_cores: usize
}

impl CPU {
    fn new(system: &System) -> CPU {
        let usage_history = Vec::<u64>::new();
        let current_usage: u64 = 0;
        let num_cores: usize = system.get_processor_list().len() - 1;
        CPU {
            usage_history,
            current_usage,
            num_cores
        }
    }
    
    fn update(&mut self, system: &mut System) {
        system.refresh_all();
        let cpu_usage = (system.get_processor_list()[0].get_cpu_usage() * 100.0).round() as u64;
        self.current_usage = cpu_usage;
        self.usage_history.push(self.current_usage);
    }
}


fn main() -> Result<(), failure::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    let events = Events::new();
    // let spork = match Spork::new() {
    //     Ok(s) => s,
    //     Err(e) => panic!("spork error: {:?}", e)
    // };
    // let spork_stats = spork.stats(StatType::Process).unwrap();
    // let mut cpu = spork_stats.cpu;
    // let mut mem = spork.stats_with_cpus(StatType::Process, Some(4)).unwrap().memory;
    // let mut maxMem = spork.

    let mut system = sysinfo::System::new();
    let mut cpu = CPU::new(&system);

    loop {
        cpu.update(&mut system);

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
                        .title(&format!("Usage: {}% | Number of Cores: {}", cpu.current_usage, cpu.num_cores))
                        .borders(Borders::LEFT | Borders::RIGHT)
                )
                .data(&cpu.usage_history.as_slice())
                .style(Style::default().fg(Color::Yellow))
                .render(&mut f, chunks[2]);
            // let log = cpu.usage_history.iter()
            //     .map(|cpu|Text::raw(format!("{}%", cpu)));
            // List::new(log)
            //     .block(
            //         Block::default()
            //             .borders(Borders::ALL)
            //             .title("Log")
            //     )
            //     .render(&mut f, chunks[3])
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => break,
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}
