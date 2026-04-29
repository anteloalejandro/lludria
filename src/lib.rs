use std::collections::HashMap;
/// # INFO
/// <https://github.com/PyO3/pyo3#using-python-from-rust>
use std::{ffi::CStr, path::PathBuf, time::Duration};
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::PyDict;

const WRAPPER_MODULE: &CStr = c"wrapper";
const WRAPPER_FILENAME: &CStr = c"wrapper.py";
static WRAPPER: &CStr = c_str!(include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/wrapper.py"
)));

fn games() -> PyResult<Vec<Game>> {
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
            .iter().map(|dict| -> PyResult<Game> {
                let map: HashMap<String, Py<PyAny>> = dict.extract(py)?;
                Ok(Game {
                    id: map.get("id").unwrap().extract(py)?,
                    name: map.get("name").unwrap().extract(py)?,
                    slug: map.get("slug").unwrap().extract(py)?,
                    ..Default::default()
                })
                
            })
            .collect::<PyResult<Vec<_>>>()?;

        PyResult::Ok(games)
    })?;

    Ok(games)
}

#[derive(Debug, Default)]
struct Game {
    id: i32,
    name: String,
    slug: String,
    cover: Option<PathBuf>,
    banner: Option<PathBuf>,
    icon: Option<PathBuf>,
    playtime: Option<Duration>,
    last_played: Option<Duration>,
    run_command: Option<String>,
    is_running: bool
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_games() {
        dbg!(games().unwrap());
    }
}
