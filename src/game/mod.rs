use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CStr;
use std::io;
use std::process::{Child, ChildStderr, ChildStdout, Command};
use std::os::unix::process::CommandExt;
use std::rc::Rc;
use std::{path::PathBuf, time::Duration};
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use chrono::{DateTime, Utc};

mod platform;

pub use platform::Platform;

const WRAPPER_MODULE: &CStr = c"wrapper";
const WRAPPER_FILENAME: &CStr = c"wrapper.py";
static WRAPPER: &CStr = c_str!(include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/wrapper.py"
)));

#[derive(Debug, Clone)]
pub struct Game {
    /// Game ID, managed automatically by lutris.
    pub id: i32,
    /// Game name, set by the user.
    pub name: String,
    /// Game slug, unique between games.
    pub slug: String,
    /// Cover art image absolute path.
    pub cover: Option<PathBuf>,
    /// Banner/Hero image absolute path.
    pub banner: Option<PathBuf>,
    /// Desktop Icon image absolute path.
    pub icon: Option<PathBuf>,
    /// Time played.
    pub playtime: Option<Duration>,
    /// Timestamp of the last time played.
    pub last_played: Option<DateTime<Utc>>,
    /// List of categories the game belongs to. The category used for hidden games is `.hidden` .
    pub categories: Vec<String>,
    /// Platform the game runs on, based on its runner.
    pub platform: Platform,
    /// Command to execute the game. Currently based on the lutris cli.
    pub run_command: String,
    /// Process ran using [`Game::run`].
    ///
    /// *If another Lutris instance was already opened, all children of the running process are
    /// managed by said instance and this running process is closed (thus, set to `None`) automatically.*
    running_process: Option<Rc<RefCell<Child>>>
}
impl Game {
    /// Get a list of installed games.
    ///
    /// Returns a [`PyErr`] if there is **any** errors parsing the game library.
    pub fn installed() -> PyResult<Vec<Game>> {
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

    /// Takes in a python `dict` with all the fields in the [`Game`] struct and creates an instance
    /// of said struct.
    ///
    /// See the source code for `wrapper.py` in the repo for this library for more information on
    /// the expected format of the python `dict`.
    fn from_py_dict(py: Python, dict: Py<PyDict>) -> PyResult<Self> {
        let map: HashMap<String, Py<PyAny>> = dict.extract(py)?;

        // the `playtime` stat is in float format, with the whole part being the hours
        let playtime = map.get("playtime").unwrap()
            .extract::<f32>(py)
            .map(|playtime| {
                let hours = playtime.floor();
                let minutes = (playtime - hours) * 60.0;
                Duration::from_secs_f32(hours * 3600. + minutes * 60.)
            })
            .ok();
        // the `last_played` stat is a timestamp in seconds
        let last_played = map.get("last_played").unwrap()
            .extract::<u32>(py)
            .map(|timestamp| DateTime::from_timestamp_secs(timestamp as i64).unwrap())
            .ok();

        let platform = Platform::from_runner(
            map.get("runner").unwrap().extract(py)?
        );

        Ok(Game {
            id: map.get("id").unwrap().extract(py)?,
            name: map.get("name").unwrap().extract(py)?,
            slug: map.get("slug").unwrap().extract(py)?,
            cover: map.get("cover").unwrap().extract(py)?,
            banner: map.get("banner").unwrap().extract(py)?,
            icon: map.get("icon").unwrap().extract(py)?,
            playtime,
            last_played,
            platform,
            categories: map.get("categories").unwrap().extract(py)?,
            run_command: map.get("run_command").unwrap().extract(py)?,
            running_process: None
        })
    }

    /// Run a [`Game`].
    ///
    /// Returns `Ok` if the game is running.
    pub fn run(&mut self) -> io::Result<()> {
        if self.running_process.is_some() { return Ok(()) }

        let mut command_parts = self.run_command.split_whitespace();
        let mut command = Command::new(command_parts.next().expect("command must not be empty"));
        let command = command
            .args(command_parts)
            // NOTE: `process_group(0)` sets the PGID equal to the PID of the spawned command.
            // This allows to kill this process and all of its descendants with `killpg`
            .process_group(0); 

        self.running_process = Some(Rc::new(RefCell::new(command.spawn()?)));

        Ok(())
    }

    /// Stop a running [`Game`].
    ///
    /// Returns `Ok` if the game was not running.
    ///
    /// *Always sees the game as stopped **if** there is another Lutris instance running.*
    ///
    /// See [`Game::running_process`] for more information.
    pub fn stop(&mut self) -> io::Result<(Option<ChildStdout>, Option<ChildStderr>)> {
        let Some(process) = self.running_process.take() else {
            return Ok((None, None)) 
        };

        let mut process = process.borrow_mut();
        // # NOTE: `killpg` kills the process and all its descendants
        // instead of leaving them orphaned, which would them running
        // # TODO: Find a better way to do this, ideally with safe rust
        unsafe { libc::killpg(process.id() as i32, libc::SIGKILL); }
        process.kill()?; // call `kill` just in case
        Ok((process.stdout.take(), process.stderr.take()))
    }

    /// Check if the game is currently running.
    ///
    /// *Always sees the game as stopped **if** there is another Lutris instance running.*
    ///
    /// See [`Game::running_process`] for more information.
    pub fn is_running(&self) -> bool {
        self.running_process.is_some()
    }
}

