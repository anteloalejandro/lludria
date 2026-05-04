//! A rust library to interact with lutris games.
//!
//! # About `lutris` and `python`
//!
//! This library acts as a bridge between Lutris' python modules and methods, and Rust.
//!
//! Thus, it requires a python shared library that can "see" your lutris installation.
//!
//! For more information on Python-to-Rust interop check [the pyO3's manual section on the
//! matter][pyo3-python-to-rust].
//!
//! [pyo3-python-to-rust]: https://github.com/PyO3/pyo3#using-python-from-rust

mod game;
pub use game::Game;
pub use game::Platform;
pub use pyo3::PyResult;
pub use pyo3::PyErr;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_games() {
        dbg!(Game::installed().unwrap());
    }
}
