#![allow(warnings)]

use std::str::FromStr;

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
            if let SortDirection::DESC = $sort_order {
                sorted.reverse();
            }

            sorted.iter()
                .map(|process| process.format())
                .collect()
        }
    };
}

macro_rules! hashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = hashmap!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}

pub enum SortBy {
    PID,
    Name,
    CPU,
    Memory
}

impl FromStr for SortBy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pid" => Ok(SortBy::PID),
            "name" => Ok(SortBy::Name),
            "cpu" => Ok(SortBy::CPU),
            "mem" => Ok(SortBy::Memory),
            _ => Err(())
        }
    }
}

pub enum SortDirection {
    ASC,
    DESC
}

pub enum Mode {
    Console,
    Main
}
