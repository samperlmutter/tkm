use crate::App;
use crate::util::SortBy;

pub enum Command<'a, 'b> {
    Sort(Vec<&'a str>, &'b mut App)
}

impl<'a, 'b> Command<'a, 'b> {
    // Executs a command
    pub fn exec(&mut self) -> String {
        // Matches self to relevant command
        match self {
            Command::Sort(columns, app) => {
                // Ensures that sort is only being passed one argument
                if columns.len() == 1 {
                    // Parses the &str argument into a SortBy enum
                    if let Ok(sort_by) = columns[0].parse::<SortBy>() {
                        app.processes_sort_by = sort_by;
                        app.toggle_sort_direction();
                        String::new()
                    } else {
                        "Error during sort".to_string()
                    }
                } else {
                    "Invalid number of arguments".to_string()
                }
            }
        }
    }
}
