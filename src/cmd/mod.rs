mod error;

pub use error::CmdError;

use crate::util::{ SortBy, SortDirection };

// TODO: Add quit and help commands
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
