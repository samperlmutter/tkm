use std::fmt::Display;

pub struct Console {
    pub visible: bool,
    pub history: Vec<String>,
    pub input: String
}

impl Console {
    pub fn new() -> Console {
        Console {
            history: Vec::<String>::new(),
            visible: false,
            input: String::new()
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

    pub fn append_input(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn clear_input(&mut self) -> String {
        let buffer = self.input.drain(..).collect::<String>();
        self.write(buffer.clone());
        buffer
    }

    pub fn backspace(&mut self) {
        self.input.pop();
    }
}
