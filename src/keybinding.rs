use core::fmt;

pub struct KeyBinding {
    name: String,
    key: char,
    action: Action,
}

pub enum Action {
    Command(String),
    Menu(Vec<KeyBinding>),
}

impl fmt::Display for KeyBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{key} -> {left_bracket}{name}{right_bracket}",
            key = self.key,
            name = self.name,
            left_bracket = match self.action {
                Action::Command(_) => "",
                Action::Menu(_) => "[",
            },
            right_bracket = match self.action {
                Action::Command(_) => "",
                Action::Menu(_) => "]",
            }
        )
    }
}

impl KeyBinding {
    pub fn length(&self) -> u8 {
        let len_braces = match self.action {
            Action::Command(_) => 0,
            Action::Menu(_) => 2,
        };
        let len_name = 0;
        5 + len_braces + len_name
    }
}
