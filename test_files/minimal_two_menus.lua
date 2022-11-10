twk.set_config(
    twk.config:new(
        twk.menu:new("<C-A>", "menu")
        :add_menu(
            twk.menu:new("a", "a-menu")
        ),
        "/opt/homebrew/bin/fish"
    )
)
