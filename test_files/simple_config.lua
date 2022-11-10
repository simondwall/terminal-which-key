twk.set_config(twk.config:new(
    twk.menu:new("<C-A>", "menu")
        :with_description("This is just for testing")
        :add_action(twk.action:new(
            "h",
            "Hello, World!",
            function()
                print("Hello, World!")
            end
        )
    ),
    "/opt/homebrew/bin/fish"
))
