use dirs::home_dir;
use log::{info, warn};
use serde::Deserialize;
use std::fs::read_to_string;
use termion::color;
use termion::event::Key;

#[derive(Deserialize, Debug)]
struct TomlConfig {
    borders: Option<Borders>,
    colours: Option<Colours>,
    keys: Option<Keys>,
}

#[derive(Deserialize, Debug)]
struct Borders {
    hline: Option<String>,
    vline: Option<String>,
    ulcorner: Option<String>,
    urcorner: Option<String>,
    llcorner: Option<String>,
    lrcorner: Option<String>,
}

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

pub struct Config<'a> {
    pub hline: &'a str,
    pub vline: &'a str,
    pub ulcorner: &'a str,
    pub urcorner: &'a str,
    pub llcorner: &'a str,
    pub lrcorner: &'a str,
    pub colour0: &'a dyn color::Color,
    pub colour1: &'a dyn color::Color,
    pub colour2: &'a dyn color::Color,
    pub colour3: &'a dyn color::Color,
    pub colour4: &'a dyn color::Color,
    pub colour5: &'a dyn color::Color,
    pub colour6: &'a dyn color::Color,
    pub colour7: &'a dyn color::Color,
    pub colourfg: &'a dyn color::Color,
    pub colourbg: &'a dyn color::Color,
    pub quit: Key,
    pub back: Key,
    pub save: Key,
    pub add: Key,
    pub edit: Key,
    pub delete: Key,
    pub task_up: Key,
    pub task_down: Key,
    pub up: Key,
    pub down: Key,
    pub focus: Key,
    pub complete: Key,
    pub increase: Key,
    pub decrease: Key,
}

impl<'a> Config<'a> {
    // Create default configuration
    pub fn default() -> Config<'static> {
        // Default border characters
        let hline = "─";
        let vline = "│";
        let ulcorner = "┌";
        let urcorner = "┐";
        let llcorner = "└";
        let lrcorner = "┘";

        // Default ANSI terminal colours
        let colour0 = &color::Black;
        let colour1 = &color::Red;
        let colour2 = &color::Green;
        let colour3 = &color::Yellow;
        let colour4 = &color::Blue;
        let colour5 = &color::Magenta;
        let colour6 = &color::Cyan;
        let colour7 = &color::White;

        // Default foreground and background colours
        let colourfg = &color::Reset;
        let colourbg = &color::Reset;

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

pub struct ConfigBuffer {
    pub hline: Option<String>,
    pub vline: Option<String>,
    pub ulcorner: Option<String>,
    pub urcorner: Option<String>,
    pub llcorner: Option<String>,
    pub lrcorner: Option<String>,
    pub colour0: Option<color::Rgb>,
    pub colour1: Option<color::Rgb>,
    pub colour2: Option<color::Rgb>,
    pub colour3: Option<color::Rgb>,
    pub colour4: Option<color::Rgb>,
    pub colour5: Option<color::Rgb>,
    pub colour6: Option<color::Rgb>,
    pub colour7: Option<color::Rgb>,
    pub colourfg: Option<color::Rgb>,
    pub colourbg: Option<color::Rgb>,
    pub quit: Option<Key>,
    pub back: Option<Key>,
    pub save: Option<Key>,
    pub add: Option<Key>,
    pub edit: Option<Key>,
    pub delete: Option<Key>,
    pub task_up: Option<Key>,
    pub task_down: Option<Key>,
    pub up: Option<Key>,
    pub down: Option<Key>,
    pub focus: Option<Key>,
    pub complete: Option<Key>,
    pub increase: Option<Key>,
    pub decrease: Option<Key>,
}

impl ConfigBuffer {
    pub fn config<'a>(&'a self, default: Config<'a>) -> Config<'a> {
        macro_rules! choose_config {
            ($attr:ident, $name:expr) => {{
                match &self.$attr {
                    Some(val) => {
                        info!("Using custom {}.", $name);
                        val
                    }
                    None => default.$attr,
                }
            }};
        }

        // Borders
        let hline = choose_config!(hline, "hline");
        let vline = choose_config!(vline, "vline");
        let ulcorner = choose_config!(ulcorner, "ulcorner");
        let urcorner = choose_config!(urcorner, "urcorner");
        let llcorner = choose_config!(llcorner, "llcorner");
        let lrcorner = choose_config!(lrcorner, "lrcorner");

        // Colours
        let colour0 = choose_config!(colour0, "colour0");
        let colour1 = choose_config!(colour1, "colour1");
        let colour2 = choose_config!(colour2, "colour2");
        let colour3 = choose_config!(colour3, "colour3");
        let colour4 = choose_config!(colour4, "colour4");
        let colour5 = choose_config!(colour5, "colour5");
        let colour6 = choose_config!(colour6, "colour6");
        let colour7 = choose_config!(colour7, "colour7");
        let colourfg = choose_config!(colourfg, "colourfg");
        let colourbg = choose_config!(colourbg, "colourbg");

        macro_rules! choose_config_val {
            ($attr:ident, $name:expr) => {{
                match self.$attr {
                    Some(val) => {
                        info!("Using custom {}.", $name);
                        val
                    }
                    None => default.$attr,
                }
            }};
        }

        // Keys
        let quit = choose_config_val!(quit, "quit key");
        let back = choose_config_val!(back, "back key");
        let save = choose_config_val!(save, "save key");
        let add = choose_config_val!(add, "add key");
        let edit = choose_config_val!(edit, "edit key");
        let delete = choose_config_val!(delete, "delete key");
        let task_up = choose_config_val!(task_up, "task_up key");
        let task_down = choose_config_val!(task_down, "task_down key");
        let up = choose_config_val!(up, "up key");
        let down = choose_config_val!(down, "down key");
        let focus = choose_config_val!(focus, "focus key");
        let complete = choose_config_val!(complete, "complete key");
        let increase = choose_config_val!(increase, "increase key");
        let decrease = choose_config_val!(decrease, "decrease key");

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

pub fn check_for_config() -> Option<ConfigBuffer> {
    // Check for config file at ~/.todo/config.toml
    let mut filename = match home_dir() {
        Some(dir) => dir,
        None => {
            warn!("Unable to locate home directory.");
            return None;
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
            return None;
        }
    };

    let toml_config: TomlConfig = match toml::from_str(&buffer) {
        Ok(toml) => {
            info!("Configuration parsed from file.");
            toml
        }
        Err(err) => {
            warn!("Unable to parse ~/.todo/config.toml: {}", err);
            return None;
        }
    };

    let (hline, vline, ulcorner, urcorner, llcorner, lrcorner) = match toml_config.borders {
        Some(border) => (
            border.hline,
            border.vline,
            border.ulcorner,
            border.urcorner,
            border.llcorner,
            border.lrcorner,
        ),
        None => (None, None, None, None, None, None),
    };
    let (
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
    ) = match toml_config.colours {
        Some(colours) => (
            colours.colour0,
            colours.colour1,
            colours.colour2,
            colours.colour3,
            colours.colour4,
            colours.colour5,
            colours.colour6,
            colours.colour7,
            colours.colourfg,
            colours.colourbg,
        ),
        None => (None, None, None, None, None, None, None, None, None, None),
    };

    let (
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
    ) = match toml_config.keys {
        Some(keys) => (
            keys.quit,
            keys.back,
            keys.save,
            keys.add,
            keys.edit,
            keys.delete,
            keys.task_up,
            keys.task_down,
            keys.up,
            keys.down,
            keys.focus,
            keys.complete,
            keys.increase,
            keys.decrease,
        ),
        None => (
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        ),
    };

    Some(ConfigBuffer {
        hline,
        vline,
        ulcorner,
        urcorner,
        llcorner,
        lrcorner,
        colour0: colour0.map(|x| color::Rgb(x[0], x[1], x[2])),
        colour1: colour1.map(|x| color::Rgb(x[0], x[1], x[2])),
        colour2: colour2.map(|x| color::Rgb(x[0], x[1], x[2])),
        colour3: colour3.map(|x| color::Rgb(x[0], x[1], x[2])),
        colour4: colour4.map(|x| color::Rgb(x[0], x[1], x[2])),
        colour5: colour5.map(|x| color::Rgb(x[0], x[1], x[2])),
        colour6: colour6.map(|x| color::Rgb(x[0], x[1], x[2])),
        colour7: colour7.map(|x| color::Rgb(x[0], x[1], x[2])),
        colourfg: colourfg.map(|x| color::Rgb(x[0], x[1], x[2])),
        colourbg: colourbg.map(|x| color::Rgb(x[0], x[1], x[2])),
        quit: quit.map(|x| Key::Char(x)),
        back: back.map(|x| Key::Char(x)),
        save: save.map(|x| Key::Char(x)),
        add: add.map(|x| Key::Char(x)),
        edit: edit.map(|x| Key::Char(x)),
        delete: delete.map(|x| Key::Char(x)),
        task_up: task_up.map(|x| Key::Char(x)),
        task_down: task_down.map(|x| Key::Char(x)),
        up: up.map(|x| Key::Char(x)),
        down: down.map(|x| Key::Char(x)),
        focus: focus.map(|x| Key::Char(x)),
        complete: complete.map(|x| Key::Char(x)),
        increase: increase.map(|x| Key::Char(x)),
        decrease: decrease.map(|x| Key::Char(x)),
    })
}
