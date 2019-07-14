use dirs::home_dir;
use log::{info, warn};
use serde::Deserialize;
use std::fs::read_to_string;
use termion::color;

#[derive(Deserialize)]
struct TomlConfig {
    borders: Borders,
    colours: Colours,
}

#[derive(Deserialize)]
struct Borders {
    hline: String,
    vline: String,
    ulcorner: String,
    urcorner: String,
    llcorner: String,
    lrcorner: String,
}

#[derive(Deserialize)]
struct Colours {
    colour0: (u8, u8, u8),
    colour1: (u8, u8, u8),
    colour2: (u8, u8, u8),
    colour3: (u8, u8, u8),
    colour4: (u8, u8, u8),
    colour5: (u8, u8, u8),
    colour6: (u8, u8, u8),
    colour7: (u8, u8, u8),
    colourfg: (u8, u8, u8),
    colourbg: (u8, u8, u8),
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
}

impl<'a> Config<'a> {
    // Create default configuration
    pub fn default() -> Config<'static> {
        // Border characters
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
        }
    }
}

pub fn check_for_config() -> Result<Config<'static>, ()> {
    // Check for config file at ~/.todo/config.toml
    let mut filename = match home_dir() {
        Some(dir) => dir,
        None => {
            warn!("Unable to locate home directory.");
            return Err(());
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
            return Err(());
        }
    };

    let toml_config: TomlConfig = match toml::from_str(&buffer) {
        Ok(toml) => {
            info!("Configuration parsed from file.");
            toml
        }
        Err(err) => {
            warn!("Unable to parse ~/.todo/config.toml: {}", err);
            return Err(());
        }
    };

    // TODO
    Ok(Config::default())
}
