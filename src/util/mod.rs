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

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SortBy {
    PID,
    Name,
    CPU,
    Memory,
}

impl FromStr for SortBy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pid" => Ok(SortBy::PID),
            "name" => Ok(SortBy::Name),
            "cpu" => Ok(SortBy::CPU),
            "mem" => Ok(SortBy::Memory),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SortDirection {
    ASC,
    DESC,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Mode {
    Console,
    Main,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CmdError<'a> {
    IncorrectArgNum(u32, u32),
    InvalidCmd(&'a str),
    InvalidArg(&'a str),
    Err(&'a str),
    ParseErr,
}

impl<'a> From<u32> for CmdError<'a> {
    fn from(i: u32) -> Self {
        CmdError::ParseErr
    }
}

impl<'a> CmdError<'a> {
    pub fn display(&self) -> String {
        match self {
            CmdError::IncorrectArgNum(exp, rec) => {
                format!("Wrong number of arguments: expected {}, found {}", exp, rec)
            }
            CmdError::InvalidCmd(cmd) => format!("Command not found: {}", cmd),
            CmdError::InvalidArg(arg) => format!("Invalid argument: {}", arg),
            CmdError::Err(err) => format!("Error: {}", err),
            CmdError::ParseErr => format!("Error during parsing"),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Action {
    Sort,
    Kill
}

#[derive(PartialEq, Debug, Clone)]
pub struct Cmd<'a> {
    pub cmd: Action,
    pub args: Vec<nom::types::CompleteStr<'a>>
}

impl<'a> Cmd<'a> {
    pub fn exec(&self, app: &mut crate::app::App) -> Result<(), CmdError> {
        match self.cmd {
            Action::Sort => {
                // Make sure only 1 argument is supplied
                if self.args.len() != 1 {
                    return Err(CmdError::IncorrectArgNum(1, self.args.len() as u32));
                }
                // Parse the first (and only) argument into a SortBy enum
                match self.args[0].0.parse::<SortBy>() {
                    Ok(sort_by) => {
                        if app.processes_sort_by == sort_by {
                            app.processes_sort_direction = if app.processes_sort_direction == SortDirection::ASC {
                                SortDirection::DESC
                            } else {
                                SortDirection::ASC
                            };
                        } else {
                            app.processes_sort_direction = SortDirection::DESC;
                        }
                        app.processes_sort_by = sort_by;
                    }
                    // If parsing failed, an invalid argument was supplied
                    Err(()) => return Err(CmdError::InvalidArg(self.args[0].0))
                }
            }
            Action::Kill => {
                // Make sure only 1 argument is supplied
                if self.args.len() != 1 {
                    return Err(CmdError::IncorrectArgNum(1, self.args.len() as u32));
                }
                match self.args[0].0.parse::<i32>() {
                    Ok(pid) => app.system.kill_process(pid),
                    Err(_) => return Err(CmdError::Err("Invalid PID"))
                }
            }
        }

        Ok(())
    }
}
