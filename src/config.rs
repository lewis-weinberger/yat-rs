use dirs::home_dir;
use log::{info, warn};
use serde::Deserialize;
use std::fs::read_to_string;
use termion::color;

#[derive(Deserialize, Debug)]
struct TomlConfig {
    borders: Borders,
    colours: Colours,
}

#[derive(Deserialize, Debug)]
struct Borders {
    hline: String,
    vline: String,
    ulcorner: String,
    urcorner: String,
    llcorner: String,
    lrcorner: String,
}

#[derive(Deserialize, Debug)]
struct Colours {
    colour0: Vec<u8>,
    colour1: Vec<u8>,
    colour2: Vec<u8>,
    colour3: Vec<u8>,
    colour4: Vec<u8>,
    colour5: Vec<u8>,
    colour6: Vec<u8>,
    colour7: Vec<u8>,
    colourfg: Vec<u8>,
    colourbg: Vec<u8>,
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

pub struct ConfigBuffer {
    pub hline: String,
    pub vline: String,
    pub ulcorner: String,
    pub urcorner: String,
    pub llcorner: String,
    pub lrcorner: String,
    pub colour0: color::Rgb,
    pub colour1: color::Rgb,
    pub colour2: color::Rgb,
    pub colour3: color::Rgb,
    pub colour4: color::Rgb,
    pub colour5: color::Rgb,
    pub colour6: color::Rgb,
    pub colour7: color::Rgb,
    pub colourfg: color::Rgb,
    pub colourbg: color::Rgb,
}

impl ConfigBuffer {
    pub fn config(&self) -> Config<'_> {
        Config {
            hline: &self.hline,
            vline: &self.vline,
            ulcorner: &self.ulcorner,
            urcorner: &self.urcorner,
            llcorner: &self.llcorner,
            lrcorner: &self.lrcorner,
            colour0: &self.colour0,
            colour1: &self.colour1,
            colour2: &self.colour2,
            colour3: &self.colour3,
            colour4: &self.colour4,
            colour5: &self.colour5,
            colour6: &self.colour6,
            colour7: &self.colour7,
            colourfg: &self.colourfg,
            colourbg: &self.colourbg,
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

    Some(ConfigBuffer {
        hline: toml_config.borders.hline,
        vline: toml_config.borders.vline,
        ulcorner: toml_config.borders.ulcorner,
        urcorner: toml_config.borders.urcorner,
        llcorner: toml_config.borders.llcorner,
        lrcorner: toml_config.borders.lrcorner,
        colour0: colour_from_vec(toml_config.colours.colour0),
        colour1: colour_from_vec(toml_config.colours.colour1),
        colour2: colour_from_vec(toml_config.colours.colour2),
        colour3: colour_from_vec(toml_config.colours.colour3),
        colour4: colour_from_vec(toml_config.colours.colour4),
        colour5: colour_from_vec(toml_config.colours.colour5),
        colour6: colour_from_vec(toml_config.colours.colour6),
        colour7: colour_from_vec(toml_config.colours.colour7),
        colourfg: colour_from_vec(toml_config.colours.colourfg),
        colourbg: colour_from_vec(toml_config.colours.colourbg),
    })
}

fn colour_from_vec(rgb: Vec<u8>) -> color::Rgb {
    color::Rgb(rgb[0], rgb[1], rgb[2])
}
