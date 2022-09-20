use pancurses::Input;

pub struct KeymapBinding {
    character: String,
    description: String,
    alt_character: Option<String>,
}

impl KeymapBinding {
    pub fn new(character: &str, description: &str) -> KeymapBinding {
        KeymapBinding {
            character: character.to_string(),
            description: description.to_string(),
            alt_character: None,
        }
    }

    pub fn new_with_alt(character: &str, description: &str, alt_character: &str) -> KeymapBinding {
        KeymapBinding {
            character: character.to_string(),
            description: description.to_string(),
            alt_character: Some(alt_character.to_string()),
        }
    }
}

pub enum Keymap {
    Quit,
    Help,
    Start,
    Up,
    Down,
    Left,
    Right,
    Unknown,
}

impl Keymap {
    pub fn from_input(input: Input) -> Keymap {
        match input {
            Input::Character('q') => Keymap::Quit,
            Input::Character('?') => Keymap::Help,
            Input::Character('n') => Keymap::Start,
            Input::Character('k') => Keymap::Up,
            Input::Character('j') => Keymap::Down,
            Input::Character('h') => Keymap::Left,
            Input::Character('l') => Keymap::Right,
            _ => Keymap::Unknown,
        }
    }

    pub fn binding(&self) -> KeymapBinding {
        match self {
            Self::Quit => KeymapBinding::new("q", "Quits the game."),
            Self::Help => KeymapBinding::new("?", "Brings up rules and keymaps."),
            Self::Start => KeymapBinding::new("n", "Starts a new game."),
            Self::Up => KeymapBinding::new_with_alt("k", "Moves cursor up.", "Up-Arrow"),
            Self::Down => KeymapBinding::new_with_alt("j", "Moves cursor down.", "Down-Arrow"),
            Self::Left => KeymapBinding::new_with_alt("h", "Moves cursor left.", "Left-Arrow"),
            Self::Right => KeymapBinding::new_with_alt("l", "Moves cursor right.", "Right-Arrow"),
            Self::Unknown => KeymapBinding::new("", ""),
        }
    }

    pub fn key(&self) -> String {
        let binding = self.binding();
        let mut message = format!("{}", binding.character);

        match binding.alt_character {
            Some(character) => message.push_str(&format!(" (Alt: {})", character)),
            None => (),
        }

        message
    }

    pub fn help_message(&self) -> String {
        let binding = self.binding();
        let mut message = format!("{}    {}", binding.character, binding.description);

        match binding.alt_character {
            Some(character) => message.push_str(&format!(" (Alt: {})", character)),
            None => (),
        }

        message
    }
}
