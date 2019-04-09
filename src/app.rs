use crate::util::*;
use crate::console::Console;
use crate::command::Command;

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

    pub fn process_command(&mut self) {
        let input = self.console.clear_input();
        if let Some((cmd, args)) = input.split_ascii_whitespace().collect::<Vec<&str>>().split_first() {
            match *cmd {
                "sort" => Command::Sort(args[0], self).exec(),
                _ => self.console.write(format!("Command not found: {}", cmd))
            }
        } else {
            self.console.write("Error parsing command".to_string());
        }
    }
}
