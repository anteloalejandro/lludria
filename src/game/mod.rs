use std::collections::HashMap;
use std::io;
use std::process::{Child, ChildStderr, ChildStdout, Command};
use std::os::unix::process::CommandExt;
use std::{path::PathBuf, time::Duration};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use chrono::{DateTime, Utc};

mod platform;
use platform::Platform;

#[derive(Debug)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub cover: Option<PathBuf>,
    pub banner: Option<PathBuf>,
    pub icon: Option<PathBuf>,
    pub playtime: Option<Duration>,
    pub last_played: Option<DateTime<Utc>>,
    pub platform: Platform,
    pub run_command: String,
    pub running_process: Option<Child>
}
impl Game {
    pub fn from_py_dict(py: Python, dict: Py<PyDict>) -> PyResult<Self> {
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

        self.running_process = Some(command.spawn()?);

        Ok(())
    }

    /// Stop a running [`Game`].
    ///
    /// Returns `Ok` if the game was not running.
    pub fn stop(&mut self) -> io::Result<(Option<ChildStdout>, Option<ChildStderr>)> {
        let Some(mut process) = self.running_process.take() else {
            return Ok((None, None)) 
        };
        // # NOTE: `killpg` kills the process and all its descendants
        // instead of leaving them orphaned, which would them running
        // # TODO: Find a better way to do this, ideally with safe rust
        unsafe { libc::killpg(process.id() as i32, libc::SIGKILL); }
        process.kill()?;
        Ok((process.stdout, process.stderr))
    }

    pub fn is_running(&self) -> bool {
        self.running_process.is_some()
    }
}

