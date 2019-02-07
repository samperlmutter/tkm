pub struct App {
    cursor_loc: u32
}

impl App {
    pub fn new() -> App {
        App {
            cursor_loc: 0
        }
    }

    pub fn move_cursor_down(&mut self) {
        self.cursor_loc -= 1;
        // TODO: Add underflow handling
    }

    pub fn move_cursor_up(&mut self) {
        self.cursor_loc += 1;
        // TODO: Add overflow handling
    }
}
