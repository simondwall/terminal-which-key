pub mod styles {
    use crossterm::style::{Color, StyledContent};
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Styles {
        #[serde(default)]
        pub colors: Colors,

        #[serde(default)]
        pub icons: Icons,

        #[serde(default)]
        pub border: Border,
    }

    #[derive(Deserialize, Debug)]
    pub struct Colors {
        #[serde(default = "Colors::default_popup_background")]
        pub popup_background: (u8, u8, u8),
        #[serde(default = "Colors::default_popup_foreground")]
        pub popup_foreground: (u8, u8, u8),
        #[serde(default = "Colors::default_border_background")]
        pub border_background: (u8, u8, u8),
        #[serde(default = "Colors::default_border_foreground")]
        pub border_foreground: (u8, u8, u8),
        #[serde(default = "Colors::default_arrow_background")]
        pub arrow_background: (u8, u8, u8),
        #[serde(default = "Colors::default_arrow_foreground")]
        pub arrow_foreground: (u8, u8, u8),
        #[serde(default = "Colors::default_menu_background")]
        pub menu_background: (u8, u8, u8),
        #[serde(default = "Colors::default_menu_foreground")]
        pub menu_foreground: (u8, u8, u8),
    }
    impl Default for Colors {
        fn default() -> Self {
            Colors {
                popup_background: (0x00, 0x00, 0x00),
                popup_foreground: (0xff, 0xff, 0xff),
                border_background: (0x00, 0x00, 0x00),
                border_foreground: (0x10, 0x73, 0xcc),
                arrow_background: (0x00, 0x00, 0x00),
                arrow_foreground: (0x10, 0x73, 0xcc),
                menu_background: (0x00, 0x00, 0x00),
                menu_foreground: (0x10, 0x73, 0xcc),
            }
        }
    }
    impl Colors {
        fn default_popup_background() -> (u8, u8, u8) {
            (0x00, 0x00, 0x00)
        }
        fn default_popup_foreground() -> (u8, u8, u8) {
            (0xff, 0xff, 0xff)
        }
        fn default_border_background() -> (u8, u8, u8) {
            (0x00, 0x00, 0x00)
        }
        fn default_border_foreground() -> (u8, u8, u8) {
            (0x10, 0x73, 0xcc)
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

    #[derive(Deserialize, Debug)]
    pub struct Border {
        #[serde(default = "Border::default_top_edge")]
        pub top_edge: String,
        #[serde(default = "Border::default_bottom_edge")]
        pub bottom_edge: String,
        #[serde(default = "Border::default_left_edge")]
        pub left_edge: String,
        #[serde(default = "Border::default_right_edge")]
        pub right_edge: String,
        #[serde(default = "Border::default_top_left_corner")]
        pub top_left_corner: String,
        #[serde(default = "Border::default_top_right_corner")]
        pub top_right_corner: String,
        #[serde(default = "Border::default_bottom_left_corner")]
        pub bottom_left_corner: String,
        #[serde(default = "Border::default_bottom_right_corner")]
        pub bottom_right_corner: String,
    }
    impl Default for Border {
        fn default() -> Self {
            Border {
                top_edge: "─".to_owned(),
                bottom_edge: "─".to_owned(),
                left_edge: "│".to_owned(),
                right_edge: "│".to_owned(),
                top_left_corner: "╭".to_owned(),
                top_right_corner: "╮".to_owned(),
                bottom_left_corner: "╰".to_owned(),
                bottom_right_corner: "╯".to_owned(),
            }
        }
    }
    impl Border {
        fn default_top_edge() -> String {
            "─".to_owned()
        }
        fn default_bottom_edge() -> String {
            "─".to_owned()
        }
        fn default_left_edge() -> String {
            "│".to_owned()
        }
        fn default_right_edge() -> String {
            "│".to_owned()
        }
        fn default_top_left_corner() -> String {
            "╭".to_owned()
        }
        fn default_top_right_corner() -> String {
            "╮".to_owned()
        }
        fn default_bottom_left_corner() -> String {
            "╰".to_owned()
        }
        fn default_bottom_right_corner() -> String {
            "╯".to_owned()
        }
    }
    
    pub trait CrosstermColor {
        fn to_color(&self) -> Color;
    }
    
    impl CrosstermColor for (u8, u8, u8) {
        fn to_color(&self) -> Color {
            Color::Rgb { r: self.0, g: self.1, b: self.2 }
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
