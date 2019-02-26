use crate::util::*;

pub struct App {
    pub mode: Mode,
    pub processes_sort_by: SortBy,
    pub processes_sort_direction: SortDirection,
    pub size: tui::layout::Rect
}

impl App {
    pub fn new() -> App {
        App {
            mode: Mode::Main,
            processes_sort_by: SortBy::CPU,
            processes_sort_direction: SortDirection::DESC,
            size: tui::layout::Rect::new(0, 0, 0, 0)
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
}
