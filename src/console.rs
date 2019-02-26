use std::fmt::Display;

pub struct Console {
    pub visible: bool,
    pub history: Vec<String>,
    pub buffer: String
}

impl Console {
    pub fn new() -> Console {
        Console {
            history: Vec::<String>::new(),
            visible: false,
            buffer: String::new()
        }
    }

    #[allow(dead_code)]
    pub fn write<T> (&mut self, data: T)
        where
        T: Display {
        self.history.insert(0, format!("{}", data));
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }
}
