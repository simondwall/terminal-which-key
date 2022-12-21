# Terminal Which Key

A helping hand for you to remember which shortcut key does what (and it also executes them).

## Description

This is similar to how some editors which-key implementations such as emacs and nvim.
You usually define a *'Leader' key* which is used to open a menu.
In this menu are submenus and actions which you can choose from with usually a single key.

![Screenshot](https://github.com/simondwall/terminal-which-key/blob/main/doc/Screenshot.png)

## Installation

Currently there is no crate released to cargo or any package to any package manager.

You will need to install:
- lua 5.4 (And in your pkg-config or change version in Cargo.toml)
- cargo

This is why you should just be able to install it using:
```bash
cargo install --git https://github.com/simondwall/terminal-which-key.git
```

## Usage

1. Write a configuration
2. Start `terminal-which-key` in your terminal
3. Run shortcuts according to your configuration

## Configuration

A configuration is created through the lua language.

The configuration will be searched in this order:

1. At the path in `$THERMINAL_WHICH_KEY_CONFIG`
2. At `$HOME/.config/terminal-which-key/init.lua`
3. At `$HOME/.terminal-which-key/init.lua`

If no configuration is found or the configuration is wrong the program will crash as of right now.

The configuration will get a twk module injected when loading it.
This module will have functions to create a configuration through the builder pattern and a function to set the configuration.

When defining the action, a function definition has to be written.
It is usually useful to use the `os.execute()` function to run a program in the background or the `twk.write()` function to write characters to the program inside of terminal-which-key.

A little example of a configuration could be this:

```lua
twk.set_config(
    twk.config:new(
        twk.menu:new("<C-Space>", "Menu")
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
                    twk.write("echo \"Hello, World!\"\n")
                end
            )
        )
        ,
        "/bin/bash"
    )
)
```
