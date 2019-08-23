use dirs::home_dir;
use log::{info, warn};
use serde::Deserialize;
use std::fs::read_to_string;
use termion::color;

#[derive(Deserialize, Debug)]
struct TomlConfig {
    borders: Option<Borders>,
    colours: Option<Colours>,
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
}

impl ConfigBuffer {
    pub fn config<'a>(&'a self, default: Config<'a>) -> Config<'a> {
        // TODO: rewrite as macro
        let hline = match &self.hline {
            Some(string) => {
                info!("Using custom hline.");
                string
            },
            None => default.hline,
        };
        let vline = match &self.vline {
            Some(string) => {
                info!("Using custom vline.");
                string
            },
            None => default.vline,
        };
        let ulcorner = match &self.ulcorner {
            Some(string) => {
                info!("Using custom ulcorner.");
                string
            },
            None => default.ulcorner,
        };
        let urcorner = match &self.urcorner {
            Some(string) => {
                info!("Using custom urcorner.");
                string
            },
            None => default.urcorner,
        };
        let llcorner = match &self.llcorner {
            Some(string) =>  {
                info!("Using custom llcorner.");
                string
            },
            None => default.llcorner,
        };
        let lrcorner = match &self.lrcorner {
            Some(string) =>  {
                info!("Using custom lcorner.");
                string
            },
            None => default.lrcorner,
        };
        let colour0 = match &self.colour0 {
            Some(col) =>  {
                info!("Using custom colour0.");
                col
            },
            None => default.colour0,
        };
        let colour1 = match &self.colour1 {
            Some(col) =>  {
                info!("Using custom colour1.");
                col
            },
            None => default.colour1,
        };
        let colour2 = match &self.colour2 {
            Some(col) =>  {
                info!("Using custom colour2.");
                col
            },
            None => default.colour2,
        };
        let colour3 = match &self.colour3 {
            Some(col) =>  {
                info!("Using custom colour3.");
                col
            },
            None => default.colour3,
        };
        let colour4 = match &self.colour4 {
            Some(col) =>  {
                info!("Using custom colour4.");
                col
            },
            None => default.colour4,
        };
        let colour5 = match &self.colour5 {
            Some(col) =>  {
                info!("Using custom colour5.");
                col
            },
            None => default.colour5,
        };
        let colour6 = match &self.colour6 {
            Some(col) =>  {
                info!("Using custom colour6.");
                col
            },
            None => default.colour6,
        };
        let colour7 = match &self.colour7 {
            Some(col) =>  {
                info!("Using custom colour7.");
                col
            },
            None => default.colour7,
        };
        let colourfg = match &self.colourfg {
            Some(col) =>  {
                info!("Using custom colourfg.");
                col
            },
            None => default.colourfg,
        };
        let colourbg = match &self.colourbg {
            Some(col) =>  {
                info!("Using custom colourbg.");
                col
            },
            None => default.colourbg,
        };

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

    Some(ConfigBuffer {
        hline,
        vline,
        ulcorner,
        urcorner,
        llcorner,
        lrcorner,
        colour0: colour_from_vec(colour0),
        colour1: colour_from_vec(colour1),
        colour2: colour_from_vec(colour2),
        colour3: colour_from_vec(colour3),
        colour4: colour_from_vec(colour4),
        colour5: colour_from_vec(colour5),
        colour6: colour_from_vec(colour6),
        colour7: colour_from_vec(colour7),
        colourfg: colour_from_vec(colourfg),
        colourbg: colour_from_vec(colourbg),
    })
}

fn colour_from_vec(rgb: Option<Vec<u8>>) -> Option<color::Rgb> {
    match rgb {
        Some(rgbv) => Some(color::Rgb(rgbv[0], rgbv[1], rgbv[2])),
        None => None,
    }
}
