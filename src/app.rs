use crate::util::*;

pub struct App {
    pub mode: Mode,
}

impl App {
    pub fn new() -> App {
        App {
            mode: Mode::Main,
        }
    }
}
