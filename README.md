# Lludria

***Work In Progress***

Rust Interface for running and managing Lutris games.

## How this works

This crate uses `pyO3` to run [wrapper.py](wrapper.py), which calls to Lutris' native python functions, which is significantly faster than calling the `lutris` cli tool.

Because of this, [your python installation *needs* a shared library](https://github.com/PyO3/pyo3#using-python-from-rust).

## TO-DO

- [ ] Aggregate game data into a single python dict
  - [x] Id
  - [x] Name and slug
  - [ ] Platform / runner
  - [ ] Playtime and Last Played
  - [ ] Cover, banner and icon path
  - [ ] Run command
- [ ] A way to track currently running games
- [ ] Search games

## *TO-MAYBE-DO*

- [ ] Manage settings
- [ ] Uninstall / Hide games
- [ ] Add locally installed games
