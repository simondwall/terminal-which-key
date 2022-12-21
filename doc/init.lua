twk.set_config(
    twk.config:new(
        twk.menu:new("<Null>", "Menu")
        :add_action(
            twk.action:new(
                "s",
                "Split",
                function()
                    os.execute("tmux split-window")
                end
            )
        )
        :add_action(
            twk.action:new(
                "e",
                "Echo",
                function()
                    twk.write("echo Hello, World!\n")
                end
            )
        )
        :add_action(
            twk.action:new(
                "q",
                "Quit Vim",
                function()
                    twk.write(":wq\n")
                end
            )
        )
        :add_menu(
            twk.menu:new("g", "Git")
            :add_action(
                twk.action:new(
                    "s",
                    "Status",
                    function()
                        twk.write("git status\n")
                    end
                )
            )
        ),
        "/bin/zsh"
    )
)
