twk.set_config(
    twk.config:new(
        twk.menu:new("<C-a>", "menu")
        :with_description("This is just for testing")
        :with_condition(
            function()
                return true
            end
        )
        :add_action(
            twk.action:new(
                "h",
                "Hello, World!",
                function()
                    print("Hello, World!")
                end
            )
            :with_condition(
                function()
                    return false
                end
            )
        )
        :add_menu(
            twk.menu:new("<C-a>", "Test")
            :add_action(
                twk.action:new(
                    "a",
                    "Test",
                    function()
                        print("This is a test")
                    end
                )
            )
        ),
        "/opt/homebrew/bin/fish"
    )
)
