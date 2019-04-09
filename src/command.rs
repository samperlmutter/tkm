use crate::App;
use crate::util::SortBy;

pub enum Command<'a, 'b> {
    Sort(&'a str, &'b mut App)
}

impl<'a, 'b> Command<'a, 'b> {
    pub fn exec(&mut self) {
        match self {
            Command::Sort(column, app) => {
                if let Ok(sort_by) = column.parse::<SortBy>() {
                    app.processes_sort_by = sort_by;
                    app.toggle_sort_direction();
                }
            }
        }
    }
}
