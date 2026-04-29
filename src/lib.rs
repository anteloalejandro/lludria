//! # INFO
//! <https://github.com/PyO3/pyo3#using-python-from-rust>

use std::ffi::CStr;
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::PyDict;

mod game;
use game::Game;

const WRAPPER_MODULE: &CStr = c"wrapper";
const WRAPPER_FILENAME: &CStr = c"wrapper.py";
static WRAPPER: &CStr = c_str!(include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/wrapper.py"
)));

pub fn games() -> PyResult<Vec<Game>> {
    let games: Vec<Game> = Python::attach(|py| {
        let module = PyModule::from_code(
            py,
            WRAPPER,
            WRAPPER_FILENAME,
            WRAPPER_MODULE
        )?;
        let method: Py<PyAny> = module
            .getattr("get_games")?
            .into();

        let result: Vec<Py<PyDict>> = method.call0(py)?.extract(py)?;

        let games = result
            .into_iter().map(|dict| Game::from_py_dict(py, dict))
            .collect::<PyResult<Vec<_>>>()?;

        PyResult::Ok(games)
    })?;

    Ok(games)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_games() {
        dbg!(games().unwrap());
    }
}
