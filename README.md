# Lludria

***Work In Progress***

Rust Interface for running and managing Lutris games.

## How this works

This crate uses `pyO3` to run [wrapper.py](wrapper.py), which calls to Lutris' native python functions, which is significantly faster than calling the `lutris` cli tool.

Because of this, [your python installation *needs* a shared library][python-from-rust].

## TO-DO

- [x] Aggregate game data into a single python `dict[str, Any]`
  - [x] Id
  - [x] Name and slug
  - [x] Platform / runner
  - [x] Playtime and Last Played
  - [x] Cover, banner and icon path
  - [x] Run command
- [x] Track currently running games
- [x] Stop running games
- [x] Categories
- [ ] Search games

*Running and stopping games does not work properly if there is a Lutris instance opened*

## *TO-MAYBE-DO*

- [ ] Manage settings
- [ ] Uninstall / Hide games
- [ ] Add locally installed games

## Acknowledgements

This project is heavily inspired by how [`lutris-gamepad-ui`][lutris-gamepad-ui] handles the interop between Lutris' python code and JavaScript.

<!-- LINKS -->
[lutris-gamepad-ui]: https://github.com/andrew-ld/lutris-gamepad-ui
[python-from-rust]: https://github.com/PyO3/pyo3#using-python-from-rust
