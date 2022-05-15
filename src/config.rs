pub mod styles {
    use crossterm::style::Color;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Styles {
        #[serde(default)]
        pub colors: Colors,

        #[serde(default)]
        pub icons: Icons,
    }

    #[derive(Deserialize, Debug)]
    pub struct Colors {
        #[serde(default = "Colors::default_key_background")]
        pub key_background: (u8, u8, u8),
        #[serde(default = "Colors::default_key_foreground")]
        pub key_foreground: (u8, u8, u8),
        #[serde(default = "Colors::default_arrow_background")]
        pub arrow_background: (u8, u8, u8),
        #[serde(default = "Colors::default_arrow_foreground")]
        pub arrow_foreground: (u8, u8, u8),
        #[serde(default = "Colors::default_menu_background")]
        pub menu_background: (u8, u8, u8),
        #[serde(default = "Colors::default_menu_foreground")]
        pub menu_foreground: (u8, u8, u8),
        #[serde(default = "Colors::default_name_background")]
        pub name_background: (u8, u8, u8),
        #[serde(default = "Colors::default_name_foreground")]
        pub name_foreground: (u8, u8, u8),
    }
    impl Default for Colors {
        fn default() -> Self {
            Colors {
                key_background: (0x00, 0x00, 0x00),
                key_foreground: (0xff, 0xff, 0xff),
                arrow_background: (0x00, 0x00, 0x00),
                arrow_foreground: (0x10, 0x73, 0xcc),
                menu_background: (0x00, 0x00, 0x00),
                menu_foreground: (0x10, 0x73, 0xcc),
                name_background: (0x00, 0x00, 0x00),
                name_foreground: (0xff, 0xff, 0xff),
            }
        }
    }
    impl Colors {
        fn default_key_background() -> (u8, u8, u8) {
            (0x00, 0x00, 0x00)
        }
        fn default_key_foreground() -> (u8, u8, u8) {
            (0xff, 0xff, 0xff)
        }
        fn default_arrow_background() -> (u8, u8, u8) {
            (0x00, 0x00, 0x00)
        }
        fn default_arrow_foreground() -> (u8, u8, u8) {
            (0x10, 0x73, 0xcc)
        }
        fn default_menu_background() -> (u8, u8, u8) {
            (0x00, 0x00, 0x00)
        }
        fn default_menu_foreground() -> (u8, u8, u8) {
            (0x10, 0x73, 0xcc)
        }
        fn default_name_background() -> (u8, u8, u8) {
            (0x00, 0x00, 0x00)
        }
        fn default_name_foreground() -> (u8, u8, u8) {
            (0xff, 0xff, 0xff)
        }
    }

    #[derive(Deserialize, Debug)]
    pub struct Icons {
        #[serde(default = "Icons::default_arrow")]
        pub arrow: String,
        #[serde(default = "Icons::default_menu")]
        pub menu: String,
    }
    impl Default for Icons {
        fn default() -> Self {
            Icons {
                arrow: "->".to_owned(),
                menu: "...".to_owned(),
            }
        }
    }
    impl Icons {
        fn default_arrow() -> String {
            "->".to_owned()
        }
        fn default_menu() -> String {
            "...".to_owned()
        }
    }

    pub trait CrosstermColor {
        fn to_color(&self) -> Color;
    }
    impl CrosstermColor for (u8, u8, u8) {
        fn to_color(&self) -> Color {
            Color::Rgb {
                r: self.0,
                g: self.1,
                b: self.2,
            }
        }
    }
}

pub mod keybindings {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Keybinding {
        pub name: String,
        pub key: char,
        pub description: Option<String>,
        pub commands: Commands,
    }

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    pub enum Commands {
        Menu(Vec<Keybinding>),
        Command(String),
    }
}
