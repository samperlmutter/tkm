use tui::layout::{Constraint, Direction, Layout, Rect, Corner};
use tui::widgets::{Block, Borders, Widget, Sparkline, Gauge, Row, Table, List, Text, Paragraph};
use tui::style::{Color, Style};
use tui::backend::Backend;
use tui::terminal::Frame;
use pretty_bytes::converter::convert;

use crate::app::App;
use crate::util::*;

// Helper function to make creating layouts easier
pub fn define_layout (direction: Direction, constraints: &[Constraint], location: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(location)
}

pub fn render_console_layout<B> (f: &mut Frame<B>, layout: Rect, app: &App)
        where
        B: Backend {
            let log_text = app.console.history.iter().map(Text::raw);
            List::new(log_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Console")
                )
                .start_corner(Corner::BottomLeft)
                .render(f, layout);
}

pub fn render_sparklines_layout<B> (f: &mut Frame<B>, layout: &[Rect], app: &App)
    where
    B: Backend {
    Sparkline::default()
        .block(
            Block::default()
                .title(&format!("CPU Usage: {}% | Number of Cores: {}", app.system.cpu_current_usage, app.system.cpu_num_cores))
                .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                .border_style(Style::default().fg(Color::Green))
        )
        .data(&app.system.cpu_usage_history.as_slice())
        .style(Style::default().fg(Color::Yellow))
        .max(100)
        .render(f, layout[0]);

    Sparkline::default()
        .block(
            Block::default()
                .title(&format!("Memory Used: {} | Memory Free: {}", convert(app.system.mem_used as f64 * 1000.0), convert(app.system.mem_free as f64 * 1000.0)))
                .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                .border_style(Style::default().fg(Color::Green))
        )
        .data(&app.system.mem_usage_history)
        .style(Style::default().fg(Color::Blue))
        .max(app.system.mem_total)
        .render(f, layout[1]);
}

pub fn render_cpu_cores_layout<B> (f: &mut Frame<B>, layout: &[Rect], app: &App)
    where
    B: Backend {
    // Creates a guage for each cpu core
    for (i, core_usage) in app.system.cpu_core_usages.iter().enumerate() {
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

pub fn render_processes_layout<B> (f: &mut Frame<B>, layout: Rect, app: &App)
    where
    B: Backend {

    let mut processes = app.system.processes.clone();

    match app.processes_sort_by {
        SortBy::PID => processes.sort_by(|a, b| a.pid.partial_cmp(&b.pid).unwrap()),
        SortBy::Name => processes.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap()),
        SortBy::CPU => processes.sort_by(|a, b| a.cpu.partial_cmp(&b.cpu).unwrap()),
        SortBy::Memory => processes.sort_by(|a, b| a.mem.partial_cmp(&b.mem).unwrap()),
    }

    if app.processes_sort_direction == SortDirection::DESC {
        processes.reverse();
    }

    let headers = ["PID", "Name", "CPU", "Memory"];
    let fmt_processes: Vec<Vec<String>> = processes.iter().map(|process| process.format()).collect();
    let rows = fmt_processes.iter().map(|process|
        Row::Data(process.iter())
    );

    Table::new(headers.iter(), rows)
        .block(Block::default().borders(Borders::ALL).title("Processes"))
        .widths(&[6, 25, 6, 9])
        .column_spacing(4)
        .render(f, layout);
}

pub fn render_input_layout<B> (f: &mut Frame<B>, layout: Rect, app: &App)
    where B: Backend {
    Paragraph::new([Text::raw(&app.console.input)].iter())
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Input"))
        .render(f, layout);
}
