use std::fmt::Display;

pub struct Log {
    pub log: Vec<String>,
    pub show_log: bool
}

impl Log {
    pub fn new() -> Log {
        Log {
            log: Vec::<String>::new(),
            show_log: false
        }
    }

    #[allow(dead_code)]
    pub fn write<T> (&mut self, data: T) 
        where
        T: Display {
        self.log.insert(0, format!("{}", data));
    }

    pub fn toggle_log(&mut self) {
        self.show_log = !self.show_log;
    }
}
