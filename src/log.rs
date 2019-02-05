use std::fmt::Display;
use tui::layout::{Corner, Rect};
use tui::widgets::{Block, Borders, Widget, Text, List};
use tui::backend::Backend;
use tui::terminal::Frame;

pub struct Log {
    pub log: Vec<String>,
    pub show_log: bool
}

impl Log {
    pub fn new() -> Log {
        Log {
            log: Vec::<String>::new(),
            show_log: false
        }
    }

    #[allow(dead_code)]
    pub fn write<T> (&mut self, data: T) 
        where
        T: Display {
        self.log.insert(0, format!("{}", data));
    }

    pub fn render<B> (&mut self, f: &mut Frame<B>, layout: Rect) 
        where
        B: Backend {
            let log_text = self.log.iter().map(Text::raw);
            if self.show_log {
                List::new(log_text)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Log")
                    )
                    .start_corner(Corner::BottomLeft)
                    .render(f, layout);
            }
    }

    pub fn toggle_log(&mut self) {
        self.show_log = !self.show_log;
    }
}
