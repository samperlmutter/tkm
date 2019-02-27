#![allow(warnings)]

pub mod event;

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

// Macro to sort the list of processes by a variable field and format each process to be displayed
#[macro_export]
macro_rules! sort_processes {
    ($processes:expr, $struct:ident . $field:ident, $sort_order:expr) => {
        {
            let mut sorted: Vec<Process> = $processes.clone();
            sorted.sort_by(|a, b| a.$field.partial_cmp(&b.$field).unwrap());
            match $sort_order {
                SortDirection::DESC => {
                    sorted.reverse();
                }
                _ => {}
            }

            sorted.iter()
                .map(|process| process.format())
                .collect()
        }
    };
}

pub enum SortBy {
    PID,
    Name,
    CPU,
    Memory
}

pub enum SortDirection {
    ASC,
    DESC
}

pub enum Mode {
    Console,
    Main
}
