use tui::layout::{Constraint, Direction, Layout, Rect, Corner};
use tui::widgets::{Block, Borders, Widget, Sparkline, Gauge, Row, Table, List, Text};
use tui::style::{Color, Style};
use tui::backend::Backend;
use tui::terminal::Frame;
use pretty_bytes::converter::convert;

use crate::system::System;
use crate::console::*;

// Helper function to make creating layouts easier
pub fn define_layout (direction: Direction, constraints: &[Constraint], location: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(location)
}

pub fn render_console<B> (f: &mut Frame<B>, layout: Rect, console: &Console)
        where
        B: Backend {
            let log_text = console.buffer.iter().map(Text::raw);
            List::new(log_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Console")
                        .style(Style::default().bg(Color::Black))
                )
                .start_corner(Corner::BottomLeft)
                .render(f, layout);
}

pub fn render_sparklines_layout<B> (f: &mut Frame<B>, layout: &[Rect], system: &System) 
    where
    B: Backend {
    Sparkline::default()
        .block(
            Block::default()
                .title(&format!("CPU Usage: {}% | Number of Cores: {}", system.cpu_current_usage, system.cpu_num_cores))
                .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                .border_style(Style::default().fg(Color::Green))
        )
        .data(&system.cpu_usage_history.as_slice())
        .style(Style::default().fg(Color::Yellow))
        .max(100)
        .render(f, layout[0]);

    Sparkline::default()
        .block(
            Block::default()
                .title(&format!("Memory Used: {} | Memory Free: {}", convert(system.mem_used as f64 * 1000.0), convert(system.mem_free as f64 * 1000.0)))
                .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                .border_style(Style::default().fg(Color::Green))
        )
        .data(&system.mem_usage_history)
        .style(Style::default().fg(Color::Blue))
        .max(system.mem_total)
        .render(f, layout[1]);
}

pub fn render_cpu_cores_layout<B> (f: &mut Frame<B>, layout: &[Rect], system: &System)
    where
    B: Backend {
    // Creates a guage for each cpu core
    for (i, core_usage) in system.cpu_core_usages.iter().enumerate() {
        Gauge::default()
            .block(
                Block::default()
                .title(&format!("Core {}", i + 1))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
            )
            .style(Style::default().fg(Color::Green))
            .percent(*core_usage)
            .render(f, layout[i]);
    }
}

pub fn render_processes_layout<B> (f: &mut Frame<B>, layout: &[Rect], processes: &Vec<Vec<String>>)
    where
    B: Backend {
    let headers = ["PID", "Name", "CPU", "Memory"];
    let rows = processes.iter().map(|process|
        Row::Data(process.iter())
    );

    Table::new(headers.iter(), rows)
        .block(Block::default().borders(Borders::ALL).title("Processes"))
        .widths(&[6, 25, 6, 9])
        .column_spacing(4)
        .render(f, layout[1]);
}
