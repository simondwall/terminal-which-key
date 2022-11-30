twk.set_config(
    twk.config:new(
        twk.menu:new("<C-Space>", "Menu")
        :with_description("This is just for testing")
        :with_condition(
            function()
                return true
            end
        )
        :add_menu(
            twk.menu:new("p", "Pane")
            :add_action(
                twk.action:new(
                    "|",
                    "Split Horizontally",
                    function()
                        os.execute("zellij action new-pane -d right")
                    end
                )
            )
            :add_action(
                twk.action:new(
                    "-",
                    "Split Vertically",
                    function()
                        os.execute("zellij action new-pane -d down")
                    end
                )
            )
        )
        :add_action(
            twk.action:new(
                "j",
                "Move Focus Down",
                function()
                    os.execute("zellij action move-focus down")
                end
            )
        )
        :add_action(
            twk.action:new(
                "k",
                "Move Focus Up",
                function()
                    os.execute("zellij action move-focus up")
                end
            )
        )
        :add_action(
            twk.action:new(
                "h",
                "Move Focus Left",
                function()
                    os.execute("zellij action move-focus left")
                end
            )
        )
        :add_action(
            twk.action:new(
                "l",
                "Move Focus Right",
                function()
                    os.execute("zellij action move-focus right")
                end
            )
        ),
        "/opt/homebrew/bin/fish"
    )
)
