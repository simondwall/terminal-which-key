# Terminal Which Key

This is a work-in-progress thus far.
When finished it should be similar to other which-key implementations in editors but usable for the terminal.

# Todos:

## Version 0.1.0

- [x] Fix starting from root menu
- [x] Add going back to (root) menu
- [x] Disallow typing unused character when in menu
- [x] Remove Formatting
- [x] Add window displaying
- [ ] Add loading config from correct location
- [ ] Fix wrong "capitalization" on non-letter characters (e.g. currently "\" instead of "|")
- [ ] Fix resizing
- [ ] Fix transparent window
- [ ] Add more keys
- [ ] Add documentation
- [ ] Add example configs

# Roadmap

## Version 0.1.1

- [ ] Use graphemes for counting instead of chars
- [ ] Remove all unwraps
- [ ] Port lua config builder to rust
- [ ] Check lua config builder values

## Version 0.2.0

- [ ] Add condition support
- [ ] Add Format Options (Theming etc.)
- [ ] Add Miette for Error messages
- [ ] Add Clap?

## Version 0.3.0

- [ ] Add kitty graphics protocol support
- [ ] Add kitty keyboard protocol support
- [ ] Add different terminal support (Mostly keyboard support)
- [ ] Add different keyboard layout support (e.g. ISO)
- [ ] Add query capabilities (e.g. get_current_menu_name)
