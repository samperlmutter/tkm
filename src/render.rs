use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Borders, Widget, Sparkline, Gauge};
use tui::style::{Color, Style};
use tui::backend::Backend;
use tui::terminal::Frame;
use crate::system::System;

pub fn define_layout (direction: Direction, constraints: &[Constraint], location: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(location)
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
                .title(&format!("Memory Used: {} | Memory Free: {}", system.mem_used, system.mem_free))
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
     Gauge::default()
        .block(
            Block::default()
            .title("Core 1")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
        )
        .style(Style::default().fg(Color::Green))
        .percent(system.cpu_core_usages[0])
        .render(f, layout[0]);
     Gauge::default()
        .block(
            Block::default()
            .title("Core 2")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
        )
        .style(Style::default().fg(Color::Green))
        .percent(system.cpu_core_usages[1])
        .render(f, layout[1]);
     Gauge::default()
        .block(
            Block::default()
            .title("Core 3")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
        )
        .style(Style::default().fg(Color::Green))
        .percent(system.cpu_core_usages[2])
        .render(f, layout[2]);
     Gauge::default()
        .block(
            Block::default()
            .title("Core 4")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
        )
        .style(Style::default().fg(Color::Green))
        .percent(system.cpu_core_usages[3])
        .render(f, layout[3]);
}
