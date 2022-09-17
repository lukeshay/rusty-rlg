use io::*;
use pancurses::Input;

mod io;

fn main() {
    let ui = UserInterface::new();

    ui.clear();
    ui.print(0, 0, "Hello, World!");
    ui.refresh();

    loop {
        match ui.get_input() {
            Some(Input::Character('x')) => break,
            Some(Input::Character('c')) => {
                ui.clear();
            }
            Some(Input::Character(c)) => {
                ui.add_char(c);
            }
            Some(Input::KeyDC) => break,
            Some(input) => {
                ui.add_string(&format!("{:?}", input));
            }
            None => (),
        }
    }

    UserInterface::close();
}
