use crate::dungeon::*;
use crate::io::*;
use crate::keymaps::*;

const MAX_WIDTH: i32 = 80;
const MAX_HEIGHT: i32 = 30;
const MAX_X: i32 = MAX_WIDTH - 1;
const MAX_Y: i32 = MAX_HEIGHT - 1;

pub struct Board {
    pub ui: UserInterface,
}

fn get_center_x_start_point(content: &str) -> i32 {
    (MAX_WIDTH - content.chars().count() as i32) / 2
}

impl Board {
    pub fn new() -> Board {
        Board {
            ui: UserInterface::new(),
        }
    }

    pub fn print_welcome_screen(&self) {
        let welcome_message = "Welcome to the Rogue Like Game!";
        let created_by_message = "Created by Luke Shay - https://www.lukeshay.com/";
        let getting_started_message = &format!(
            "Press {} to start a game, {} to see the rules or {} to quit.",
            Keymap::Start.key(),
            Keymap::Help.key(),
            Keymap::Quit.key()
        );

        let welcome_message_start = get_center_x_start_point(welcome_message);
        let created_by_message_start = get_center_x_start_point(created_by_message);
        let getting_started_message_start = get_center_x_start_point(getting_started_message);

        self.ui.clear();
        self.ui.print(0, 0, &"#".repeat(MAX_WIDTH as usize));
        self.ui.print(13, welcome_message_start, welcome_message);
        self.ui
            .print(14, created_by_message_start, created_by_message);
        self.ui
            .print(16, getting_started_message_start, getting_started_message);
        self.ui.print(MAX_Y, 0, &"#".repeat(MAX_WIDTH as usize));

        for n in 0..=MAX_Y {
            self.ui.print(n, 0, "#");
            self.ui.print(n, MAX_X, "#");
        }

        self.ui.refresh();
    }

    pub fn print_help_screen(&self) {
        let help_title = "Help";
        let keymaps_title = "Keymaps";
        let separator = &"-".repeat(MAX_WIDTH as usize);

        let help_title_start = get_center_x_start_point(help_title);
        let keymaps_title_start = get_center_x_start_point(keymaps_title);

        self.ui.clear();
        self.ui.print(0, help_title_start, help_title);
        self.ui.print(
            2,
            2,
            "Here is some help information. Not sure when to put here.",
        );
        self.ui.print(4, 0, separator);
        self.ui.print(5, keymaps_title_start, keymaps_title);

        let keymaps = vec![
            Keymap::Quit,
            Keymap::Help,
            Keymap::Start,
            Keymap::Up,
            Keymap::Down,
            Keymap::Left,
            Keymap::Right,
        ];

        for (i, keymap) in keymaps.iter().enumerate() {
            self.ui
                .print(7 + i as i32, 2, keymap.help_message().as_str());
        }
    }

    pub fn print_dungeon(&self, dungeon: &Dungeon) {
        self.ui.clear();

        for (y, row) in dungeon.map.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                self.ui
                    .print(y as i32, x as i32, &format!("{}", col.character()));
            }
        }

        self.ui.refresh();
    }
}
