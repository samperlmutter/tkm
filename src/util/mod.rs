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

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CmdError {
    InvalidArgs = 1000,
    InvalidCmd,
    ParseErr
}

impl From<u32> for CmdError {
    fn from(i: u32) -> Self {
        match i {
            1000 => CmdError::InvalidArgs,
            1001 => CmdError::InvalidCmd,
            _ | 1002 => CmdError::ParseErr
        }
    }
}
// error_chain!{

//     foreign_links {
//         Nom(::nom::Err);
//     }

//     types {
//         InvalidArgs, InvalidCmd, ParseErr;
//     }

//     errors {
//         InvalidArgs(expected: u32, found: u32) {
//             description("Invalid number of arguments")
//             display("Wrong number of arguments: expected {}, found {}", expected, found)
//         }

//         InvalidCmd(cmd: String) {
//             description("Invalid command")
//             display("Invalid command: {}", cmd)
//         }

//         ParseErr(err: String) {
//             description("An error occurred while parsing")
//             display("Parsing error: {}", err)
//         }
//     }
// }

// impl From<CmdError> for ErrorKind {
//     fn from(err: CmdError) -> Self {
//         ErrorKind::
//     }
// }
