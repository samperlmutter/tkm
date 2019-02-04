use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Borders, Widget, Sparkline};
use tui::style::{Color, Style};
use tui::backend::Backend;
use tui::terminal::Frame;
use crate::system::System;

pub fn define_layout (direction: Direction, constraints: &[Constraint], location: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(direction)
        .constraints(constraints.as_ref())
        .split(location)
}

pub fn render_system_overview_layout<B> (f: &mut Frame<B>, layout: Rect, system: &System) 
    where
    B: Backend {
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
        .render(f, layout);
}
