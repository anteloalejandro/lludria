# Lludria

***Work In Progress***

Rust Interface for running and managing Lutris games.

## How this works

This crate uses `pyO3` to run [wrapper.py](wrapper.py), which calls to Lutris' native python functions, which is significantly faster than calling the `lutris` cli tool.

Because of this, [your python installation *needs* a shared library][python-from-rust].

## TO-DO

- [ ] Aggregate game data into a single python `dict[str, Any]`
  - [x] Id
  - [x] Name and slug
  - [ ] Platform / runner
  - [x] Playtime and Last Played
  - [x] Cover, banner and icon path
  - [ ] Category
  - [x] Run command
- [x] A way to track currently running games
  - [ ] Handle processes spawned by lutris replacing currently running process
- [ ] Search games

## *TO-MAYBE-DO*

- [ ] Manage settings
- [ ] Uninstall / Hide games
- [ ] Add locally installed games

## Acknowledgements

This project is heavily inspired by how [`lutris-gamepad-ui`][lutris-gamepad-ui] handles the interop between Lutris' python code and JavaScript.

[lutris-gamepad-ui]: https://github.com/andrew-ld/lutris-gamepad-ui
[python-from-rust]: https://github.com/PyO3/pyo3#using-python-from-rust
