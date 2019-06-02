use crate::util::{ Mode, SortBy, SortDirection };
use crate::cmd::CmdError;
use crate::console::Console;
use crate::parser::*;
use crate::System;

pub struct App {
    pub mode: Mode,
    pub processes_sort_by: SortBy,
    pub processes_sort_direction: SortDirection,
    pub size: tui::layout::Rect,
    pub console: Console,
    pub system: System,
    pub should_render: bool,
}

impl App {
    // Processes the current console buffer as a command
    pub fn process_command(&mut self) {
        let input = self.console.clear_input();

        match handle_cmd(nom::types::CompleteStr(&input)) {
            Ok((_, cmd)) => {
                if let Err(err) = cmd.exec(self) {
                    self.console.write(err.display());
                }
            }
            Err(e) => {
                if let nom::Err::Error(nom::Context::Code(_, nom::ErrorKind::Custom(cmd_err))) = e {
                    self.console.write(cmd_err.display());
                } else {
                    self.console.write(CmdError::ParseErr.display());
                }
            }
        }
    }
}
