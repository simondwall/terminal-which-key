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
- [x] Add loading config from correct location
- [x] Fix wrong "capitalization" (e.g. currently "\\" instead of "|")
- [x] Fix resizing
- [x] Fix transparent window
- [ ] Move to termion for keys instead of own implementation
- [ ] Add twk.write
- [ ] Add documentation
- [ ] Add example configs

# Possible Roadmap

## Version 0.1.1 (Most likely)

- [ ] Major refactor
- [ ] Use graphemes for counting instead of chars
- [ ] Make more stable (Remove unwraps)
- [ ] Port lua config builder to rust
- [ ] Check lua config builder values

## Version 0.2.0 (Likely)

- [ ] Add condition support
- [ ] Add Format Options (Theming etc.)
- [ ] Add Miette for Error messages
- [ ] Add Clap?

## Version 0.3.0 (Unlikely)

- [ ] Add kitty graphics protocol support
- [ ] Add kitty keyboard protocol support
- [ ] Add different terminal support (Mostly keyboard support)
- [ ] Add different keyboard layout support (e.g. ISO)
- [ ] Add query capabilities (e.g. get_current_menu_name)
- [ ] Add windows support
