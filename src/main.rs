use tracing_appender::rolling;

use board::*;
use dungeon::*;
use io::*;
use keymaps::*;
use screens::*;

mod board;
mod dungeon;
mod io;
mod keymaps;
mod screens;

fn main() {
    let logging_file = rolling::minutely("./logs", "logs.log");

    tracing_subscriber::fmt()
        .with_writer(logging_file)
        .with_ansi(false)
        .with_max_level(tracing::Level::TRACE)
        .init();

    let board = Board::new();
    let mut current_screen = Screen::Welcome;
    let mut current_dungeon = Dungeon::new();

    board.print_welcome_screen();

    loop {
        match board.ui.get_input() {
            Some(input) => match Keymap::from_input(input) {
                Keymap::Quit => break,
                Keymap::Help => match current_screen {
                    Screen::Welcome => {
                        board.print_help_screen();
                        current_screen = Screen::Help;
                    }
                    _ => {
                        board.print_welcome_screen();
                        current_screen = Screen::Welcome;
                    }
                },
                Keymap::Start => {
                    if current_screen == Screen::Dungeon {
                        current_dungeon = Dungeon::new();
                    }

                    board.print_dungeon(&current_dungeon);
                    current_screen = Screen::Dungeon;
                }
                _ => (),
            },
            None => (),
        }
    }

    UserInterface::close();
}
