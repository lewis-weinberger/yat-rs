/// Configuration functionality for controlling appearance and keybindings.
use dirs::home_dir;
use log::{info, warn};
use serde::Deserialize;
use std::fs::read_to_string;
use termion::color;
use termion::event::Key;

/// Layout of config.toml file.
#[derive(Deserialize, Debug)]
struct TomlConfig {
    borders: Option<Borders>,
    colours: Option<Colours>,
    keys: Option<Keys>,
}

/// Layout of [border] section of config.toml file.
#[derive(Deserialize, Debug)]
struct Borders {
    hline: Option<String>,
    vline: Option<String>,
    ulcorner: Option<String>,
    urcorner: Option<String>,
    llcorner: Option<String>,
    lrcorner: Option<String>,
}

/// Layout of [colours] section of config.toml file.
#[derive(Deserialize, Debug)]
struct Colours {
    colour0: Option<Vec<u8>>,
    colour1: Option<Vec<u8>>,
    colour2: Option<Vec<u8>>,
    colour3: Option<Vec<u8>>,
    colour4: Option<Vec<u8>>,
    colour5: Option<Vec<u8>>,
    colour6: Option<Vec<u8>>,
    colour7: Option<Vec<u8>>,
    colourfg: Option<Vec<u8>>,
    colourbg: Option<Vec<u8>>,
}

/// Layout of [keys] section of config.toml file.
#[derive(Deserialize, Debug)]
struct Keys {
    quit: Option<char>,
    back: Option<char>,
    save: Option<char>,
    add: Option<char>,
    edit: Option<char>,
    delete: Option<char>,
    task_up: Option<char>,
    task_down: Option<char>,
    up: Option<char>,
    down: Option<char>,
    focus: Option<char>,
    complete: Option<char>,
    increase: Option<char>,
    decrease: Option<char>,
}

/// Wrapper around Rgb and ANSI colours.
pub enum Colour {
    Rgb(Vec<u8>),
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Reset,
}

impl Colour {
    /// Return a String with foreground colour escape code.
    pub fn fg(&self) -> String {
        match self {
            Self::Rgb(rgb) => color::Fg(color::Rgb(rgb[0], rgb[1], rgb[2])).to_string(),
            Self::Black => color::Fg(color::Black).to_string(),
            Self::Red => color::Fg(color::Red).to_string(),
            Self::Green => color::Fg(color::Green).to_string(),
            Self::Yellow => color::Fg(color::Yellow).to_string(),
            Self::Blue => color::Fg(color::Blue).to_string(),
            Self::Magenta => color::Fg(color::Magenta).to_string(),
            Self::Cyan => color::Fg(color::Cyan).to_string(),
            Self::White => color::Fg(color::White).to_string(),
            Self::Reset => color::Fg(color::Reset).to_string(),
        }
    }

    /// Return a String with background colour escape code.
    pub fn bg(&self) -> String {
        match self {
            Self::Rgb(rgb) => color::Bg(color::Rgb(rgb[0], rgb[1], rgb[2])).to_string(),
            Self::Black => color::Bg(color::Black).to_string(),
            Self::Red => color::Bg(color::Red).to_string(),
            Self::Green => color::Bg(color::Green).to_string(),
            Self::Yellow => color::Bg(color::Yellow).to_string(),
            Self::Blue => color::Bg(color::Blue).to_string(),
            Self::Magenta => color::Bg(color::Magenta).to_string(),
            Self::Cyan => color::Bg(color::Cyan).to_string(),
            Self::White => color::Bg(color::White).to_string(),
            Self::Reset => color::Bg(color::Reset).to_string(),
        }
    }
}

/// Yat's configuration.
pub struct Config {
    /// Border configuration.
    /// Horizontal border character(s)
    pub hline: String,
    /// Vertical border character(s)
    pub vline: String,
    /// Upper left border character(s)
    pub ulcorner: String,
    /// Upper right border character(s)
    pub urcorner: String,
    /// Lower left border character(s)
    pub llcorner: String,
    /// Lower right border character(s)
    pub lrcorner: String,

    /// Colour-scheme configuration.
    /// Black colour.
    pub colour0: Colour,
    /// Red colour.
    pub colour1: Colour,
    /// Green colour.
    pub colour2: Colour,
    /// Yellow colour.
    pub colour3: Colour,
    /// Blue colour.
    pub colour4: Colour,
    /// Magenta colour.
    pub colour5: Colour,
    /// Cyan colour.
    pub colour6: Colour,
    /// White colour.
    pub colour7: Colour,
    /// Foreground colour.
    pub colourfg: Colour,
    /// Background colour.
    pub colourbg: Colour,

    /// Keybinding configuration.
    /// Key to quit yat.
    pub quit: Key,
    /// Key to return focus to parent.
    pub back: Key,
    /// Key to write list to save file.
    pub save: Key,
    /// Key to add new task.
    pub add: Key,
    /// Key to edit selected task.
    pub edit: Key,
    /// Key to delete selected task.
    pub delete: Key,
    /// Key to move selected task up.
    pub task_up: Key,
    /// Key to move selected task down.
    pub task_down: Key,
    /// Key to move selection up.
    pub up: Key,
    /// Key to move selection down.
    pub down: Key,
    /// Key to focus on selected sub-task.
    pub focus: Key,
    /// Key to mark task completed.
    pub complete: Key,
    /// Key to increase task priority.
    pub increase: Key,
    /// Key to decrease task priority.
    pub decrease: Key,
}

impl Config {
    /// Create default configuration.
    pub fn default() -> Config {
        // Default border characters
        let hline = String::from("─");
        let vline = String::from("│");
        let ulcorner = String::from("┌");
        let urcorner = String::from("┐");
        let llcorner = String::from("└");
        let lrcorner = String::from("┘");

        // Default ANSI terminal colours
        let colour0 = Colour::Black;
        let colour1 = Colour::Red;
        let colour2 = Colour::Green;
        let colour3 = Colour::Yellow;
        let colour4 = Colour::Blue;
        let colour5 = Colour::Magenta;
        let colour6 = Colour::Cyan;
        let colour7 = Colour::White;

        // Default foreground and background colours
        let colourfg = Colour::Reset;
        let colourbg = Colour::Reset;

        // Default keybindings
        let quit = Key::Char('q');
        let back = Key::Char('b');
        let save = Key::Char('w');
        let add = Key::Char('a');
        let edit = Key::Char('e');
        let delete = Key::Char('d');
        let task_up = Key::Char('u');
        let task_down = Key::Char('n');
        let up = Key::Up;
        let down = Key::Down;
        let focus = Key::Char('\n');
        let complete = Key::Char(' ');
        let increase = Key::Char('>');
        let decrease = Key::Char('<');

        Config {
            hline,
            vline,
            ulcorner,
            urcorner,
            llcorner,
            lrcorner,
            colour0,
            colour1,
            colour2,
            colour3,
            colour4,
            colour5,
            colour6,
            colour7,
            colourfg,
            colourbg,
            quit,
            back,
            save,
            add,
            edit,
            delete,
            task_up,
            task_down,
            up,
            down,
            focus,
            complete,
            increase,
            decrease,
        }
    }
}

/// Check for file at ~/.todo/config.toml and if present load
/// user configuration.
pub fn check_for_config() -> Config {
    // Default configuration
    let default = Config::default();
    
    // Check for config file at ~/.todo/config.toml
    let mut filename = match home_dir() {
        Some(dir) => dir,
        None => {
            warn!("Unable to locate home directory.");
            return default;
        }
    };
    filename.push(".todo/config.toml");

    let buffer = match read_to_string(filename) {
        Ok(buf) => {
            info!("Configuration file at ~/.todo/config.toml read!");
            buf
        }
        Err(err) => {
            warn!("Unable to read ~/.todo/config.toml: {}", err);
            return default;
        }
    };

    let toml_config: TomlConfig = match toml::from_str(&buffer) {
        Ok(toml) => {
            info!("Configuration parsed from file.");
            toml
        }
        Err(err) => {
            warn!("Unable to parse ~/.todo/config.toml: {}", err);
            return default;
        }
    };

    // Use new config if present, otherwise default config
    macro_rules! choose_config {
        ($kind:ident, $attr:ident, $func:ident, $name:expr) => {
            match &toml_config.$kind {
                Some(kind) => {
                    match &kind.$attr {
                        Some(attr) => {
                            info!("Using custom {}.", $name);
                            $func(attr.clone())
                        },
                        None => default.$attr,
                    }
                },
                None => default.$attr,
            }
        };
    }

    // Conversions between raw input and Config attribute types.
    fn border_convert(x: String) -> String { String::from(x) }
    fn colour_convert(x: Vec<u8>) -> Colour { Colour::Rgb(x) }
    fn key_convert(x: char) -> Key { Key::Char(x) }
    
    Config {
        hline: choose_config!(borders, hline, border_convert, "hline"),
        vline: choose_config!(borders, vline, border_convert, "vline"),
        ulcorner: choose_config!(borders, ulcorner, border_convert, "ulcorner"),
        urcorner: choose_config!(borders, urcorner, border_convert, "urcorner"),
        llcorner: choose_config!(borders, llcorner, border_convert, "llcorner"),
        lrcorner: choose_config!(borders, lrcorner, border_convert, "lrcorner"),
        colour0: choose_config!(colours, colour0, colour_convert, "colour0"),
        colour1: choose_config!(colours, colour1, colour_convert, "colour1"),
        colour2: choose_config!(colours, colour2, colour_convert, "colour2"),
        colour3: choose_config!(colours, colour3, colour_convert, "colour3"),
        colour4: choose_config!(colours, colour4, colour_convert, "colour4"),
        colour5: choose_config!(colours, colour5, colour_convert, "colour5"),
        colour6: choose_config!(colours, colour6, colour_convert, "colour6"),
        colour7: choose_config!(colours, colour7, colour_convert, "colour7"),
        colourfg: choose_config!(colours, colourfg, colour_convert, "colourfg"),
        colourbg: choose_config!(colours, colourbg, colour_convert, "colourbg"),
        quit: choose_config!(keys, quit, key_convert, "quit key"),
        back: choose_config!(keys, back, key_convert, "back key"),
        save: choose_config!(keys, save, key_convert, "save key"),
        add: choose_config!(keys, add, key_convert, "add key"),
        edit: choose_config!(keys, edit, key_convert, "edit key"),
        delete: choose_config!(keys, delete, key_convert, "delete key"),
        task_up: choose_config!(keys, task_up, key_convert, "task_up key"),
        task_down: choose_config!(keys, task_down, key_convert, "task_down key"),
        up: choose_config!(keys, up, key_convert, "up key"),
        down: choose_config!(keys, down, key_convert, "down key"),
        focus: choose_config!(keys, focus, key_convert, "focus key"),
        complete: choose_config!(keys, complete, key_convert, "complete key"),
        increase: choose_config!(keys, increase, key_convert, "increase key"),
        decrease: choose_config!(keys, decrease, key_convert, "decrease key"),
    }
}
