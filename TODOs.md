# Discussion

- [ ] WASM or other interpreted language plugin system and configuration
- [ ] Check vt100 crate for being independent of tmux for float window etc
    - vt100 seems to be something else. Rather use a pty crate (I think portable_pty looks good)
- [ ] Different commands on different conditions
- [ ] Display the name of the current bindings menu
- [ ] Searching (fuzzy) commands with tags


# Fixes

- [ ] Only check the condition once
- [ ] Keybindings.yaml: change "commands" into something better
- [ ] Check for multiple definitions of the same key


# Features

- [ ] Help pages using descriptions
- [ ] Color formatting adjustments
- [ ] Settings file
    - [ ] Custom path to shell executer
    - [ ] Custom label format
