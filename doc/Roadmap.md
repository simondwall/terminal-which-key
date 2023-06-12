# Version 0.1.0

- [x] Fix starting from root menu
- [x] Add going back to (root) menu
- [x] Disallow typing unused character when in menu
- [x] Remove Formatting
- [x] Add window displaying
- [x] Add loading config from correct location
- [x] Fix wrong "capitalization" (e.g. currently "\\" instead of "|")
- [x] Fix resizing
- [x] Fix transparent window
- [x] Add twk.write
- [x] Add example configs

# Possible Roadmap

## Version 0.1.1 (Most likely)

- [ ] Add documentation
- [ ] Change directory to current directory of slave (lsof can be of help)
- [ ] Use graphemes for counting instead of chars
- [ ] Make more stable (Remove unwraps)
- [ ] Port lua config builder to rust
- [ ] Check lua config builder values

## Version 0.2.0 (Likely)

- [ ] Add more keys (e.g. Umlaute)
- [ ] Add condition support
- [ ] Add Format Options (Theming etc.)
- [ ] Add Miette for Error messages

## Version 0.3.0 (Unlikely)

- [ ] Add windows support
- [ ] Add kitty graphics protocol support
- [ ] Add kitty keyboard protocol support
- [ ] Add query capabilities (e.g. get_current_menu_name)
