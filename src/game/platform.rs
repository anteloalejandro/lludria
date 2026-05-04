#[derive(Debug, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
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
    /// Maps known runners into known platforms.
    ///
    /// See [`Platform`].
    ///
    /// *There are currently some platforms incorrectly marked as `Unknown`*.
    pub fn from_runner(runner: &str) -> Self {
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

