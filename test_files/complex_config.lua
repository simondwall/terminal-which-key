twk.set_config(
    twk.config:new(
        twk.menu:new("<C-A>", "menu")
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
    :with_format(
        twk.format:new()
        :border(
            twk.border:new()
            :with_top_left_corner("+")
            :with_top_right_corner("+")
            :with_bottom_left_corner("+")
            :with_bottom_right_corner("+")
            :with_top_edge("-")
            :with_bottom_edge("-")
            :with_left_edge("|")
            :with_right_edge("|")
        )
        :key(
            twk.template:new()
            :color("#28f68b")
            :text("[")
            :placeholder("key")
            :text("]")
            :color("normal")
        )
        :arrow(
            twk.template:new()
            :text(">=>")
        )
        :name(
            twk.template:new()
            :evaluated_text(
                function()
                    return "current Time: " .. os.date()
                end
            )
            :placeholder("name")
        )
        :menu(
            twk.template:new()
            :text("[...]")
        )
    )
)
