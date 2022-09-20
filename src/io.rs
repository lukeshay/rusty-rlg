extern crate pancurses;

use pancurses::{curs_set, endwin, initscr, noecho, raw, Input, Window};

pub struct UserInterface {
    window: Window,
}

impl UserInterface {
    pub fn new() -> UserInterface {
        let window = initscr();
        window.keypad(true);
        noecho();
        raw();
        curs_set(0);

        UserInterface { window: window }
    }

    pub fn clear(&self) {
        self.window.clear();
    }

    pub fn close() {
        endwin();
    }

    pub fn get_input(&self) -> Option<Input> {
        self.window.getch()
    }

    pub fn print(&self, row: i32, column: i32, content: &str) {
        self.window.mvprintw(row, column, content);
    }

    pub fn refresh(&self) {
        self.window.refresh();
    }
}
