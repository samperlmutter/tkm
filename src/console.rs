use std::fmt::Display;

pub struct Console {
    pub buffer: Vec<String>,
    pub visible: bool
}

impl Console {
    pub fn new() -> Console {
        Console {
            buffer: Vec::<String>::new(),
            visible: false
        }
    }

    #[allow(dead_code)]
    pub fn write<T> (&mut self, data: T)
        where
        T: Display {
        self.buffer.insert(0, format!("{}", data));
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }
}
