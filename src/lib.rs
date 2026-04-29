//! # INFO
//! <https://github.com/PyO3/pyo3#using-python-from-rust>

use std::collections::HashMap;
use std::io;
use std::process::{Child, ChildStderr, ChildStdout, Command};
use std::os::unix::process::CommandExt;
use std::{ffi::CStr, path::PathBuf, time::Duration};
use libc::SIGKILL;
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use chrono::{DateTime, Utc};

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

#[derive(Debug)]
pub enum Platform {
    // PC
    LINUX, WINDOWS, DOS, STEAM,
    // NINTENDO
    SWITCH, WIIU, THREEDS, WIIGC, GBA, N64, SNES, GB, NES,
    // PLAYSTATION
    PSVITA, PS4, PSP, PS3, PS2, PS1,
    // XBOX
    XBOX360, XBOX,
    // SEGA
    DREAMCAST,
    // OTHER
    ATARI, RETROARCH, WEB, UNKNOWN
}
impl Platform {
    /// Maps known runners into known platforms
    fn from_runner(runner: &str) -> Self {
        use Platform::*;

        // TODO: Fix missing mappings (currently set to `UNKNOWN`)
        match runner {
            "atari800" => ATARI,
            "azahar" => THREEDS,
            "cemu" => WIIU,
            "dolphin" => WIIGC,
            "dosbox" => DOS,
            "duckstation" => PS1,
            "easyrpg" => UNKNOWN,
            "flatpak" => LINUX,
            "fsuae" => UNKNOWN,
            "hatari" => ATARI,
            "jzintv" => UNKNOWN,
            "libretro" => RETROARCH,
            "linux" => LINUX,
            "mame" => UNKNOWN,
            "mednafen" => UNKNOWN,
            "mupen64plus" => N64,
            "o2em" => UNKNOWN,
            "osmose" => UNKNOWN,
            "pcsx2" => PS2,
            "pico8" => UNKNOWN,
            "redream" => DREAMCAST,
            "reicast" => DREAMCAST,
            "rpcs3" => PS3,
            "ryujinx" => SWITCH,
            "scummvm" => UNKNOWN,
            "shadps4" => PS4,
            "snes9x" => SNES,
            "steam" => STEAM,
            "vice" => UNKNOWN,
            "vita3k" => PSVITA,
            "web" => WEB,
            "wine" => WINDOWS,
            "xemu" => XBOX,
            "xenia" => XBOX360,
            "yuzu" => SWITCH,
            "zdoom" => UNKNOWN,
            _ => UNKNOWN
        }
    }
}

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
        unsafe { libc::killpg(process.id() as i32, SIGKILL); }
        process.kill()?;
        Ok((process.stdout, process.stderr))
    }

    pub fn is_running(&self) -> bool {
        self.running_process.is_some()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_games() {
        dbg!(games().unwrap());
    }
}
