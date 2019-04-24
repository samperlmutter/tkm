use crate::util::*;
use crate::console::Console;
use crate::command_handler::*;

pub struct App {
    pub mode: Mode,
    pub processes_sort_by: SortBy,
    pub processes_sort_direction: SortDirection,
    pub size: tui::layout::Rect,
    pub console: Console
}

impl App {
    pub fn new() -> App {
        App {
            mode: Mode::Main,
            processes_sort_by: SortBy::CPU,
            processes_sort_direction: SortDirection::DESC,
            size: tui::layout::Rect::new(0, 0, 0, 0),
            console: Console::new()
        }
    }

    // Toggles the soring of processes between ascending and descending
    pub fn toggle_sort_direction(&mut self) {
        match self.processes_sort_direction {
            SortDirection::ASC => {
                self.processes_sort_direction = SortDirection::DESC;
            }
            SortDirection::DESC => {
                self.processes_sort_direction = SortDirection::ASC;
            }
        }
    }

    // Processes the current console buffer as a command
    pub fn process_command(&mut self) {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .append(true)
                    .open("foo.txt")
                    .expect("Unable to write data");

        let input = self.console.clear_input();

        match parse_cmd(input.as_bytes()) {
            Ok((_, cmd)) => {
                self.console.write(format!("{:?}", cmd));
            }
            Err(e) => {
                match e {
                    nom::Err::Failure(err) | nom::Err::Error(err) => {
                        match err {
                            nom::Context::Code(_, nom::ErrorKind::Custom(CmdError::InvalidArgs)) => {
                                self.console.write("Invalid number of arguments");
                            }
                            nom::Context::Code(_, nom::ErrorKind::Alt) => {
                                self.console.write("Unknown command");
                            }
                            nom::Context::Code(_, nom::ErrorKind::Custom(CmdError::ParseErr)) | _ => {
                                self.console.write("Error during parsing");
                            }
                        }
                        // file.write(format!("{} | {:?}\n", input, err).as_bytes()).expect("Unable to write data");
                    }
                    _ => {}
                }
            }
        }
        file.write(format!("{} | {:?}\n", input, parse_cmd(input.as_bytes())).as_bytes()).expect("Unable to write data");
    }
}
