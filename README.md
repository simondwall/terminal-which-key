# Terminal Which Key

A helping hand for you to remember which shortcut key does what (and it also executes them).

## Description

## Installation

## Usage

## Configuration

```lua
twk.set_config(                             -- Set this config
    twk.config:new(                         -- Create a config
        twk.menu:new("<C-Space>", "Menu")   -- Create a menu
        :add_action(                        -- Attach an action to this menu
            twk.action:new(                 -- Create an action
                "q",                        -- The key to press to run this action
                "quit",                     -- The name displayed in the which key window
                function()                  -- The function executed for this action
                    print("Test")
                    os.execute("exit")
                end
            )
        ),
        "/bin/bash"                         -- The shell to use
    )
)
```
