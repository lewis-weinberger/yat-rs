use crate::config::Config;
use log::{error, warn};
use std::io::{Stdin, Stdout, Write};
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, color, cursor, style};

pub struct Window<'a> {
    stdin: Keys<Stdin>,
    stdout: RawTerminal<Stdout>,
    config: Config<'a>,
}

impl<'a> Drop for Window<'a> {
    fn drop(&mut self) {
        self.endwin();
        self.show_cursor();
    }
}

impl<'a> Window<'a> {
    pub fn new(stdin: Stdin, stdout: Stdout, config: Config<'a>) -> Result<Window<'a>, ()> {
        let raw = match stdout.into_raw_mode() {
            Ok(out) => out,
            Err(_) => {
                error!("Unable to set terminal to raw mode.");
                return Err(());
            }
        };
        Ok(Window {
            stdin: stdin.keys(),
            stdout: raw,
            config,
        })
    }

    pub fn get_max_yx(&self) -> (usize, usize) {
        let (y, x) = termion::terminal_size().unwrap_or_else(|err| {
            warn!("Unable to determine terminal size: {}.", err);
            (0, 0)
        });
        (x as usize, y as usize)
    }

    pub fn hide_cursor(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap_or_else(|err| {
            warn!("Unable to hide cursor: {}.", err);
        });
    }

    pub fn show_cursor(&mut self) {
        write!(self.stdout, "{}", cursor::Show).unwrap_or_else(|err| {
            warn!("Unable to show cursor: {}", err);
        });
    }

    pub fn refresh(&mut self) {
        self.stdout.flush().unwrap_or_else(|err| {
            warn!("Unable to flush stdout: {}", err);
        });
    }

    pub fn getch(&mut self) -> Option<Key> {
        match self.stdin.next() {
            Some(Ok(key)) => Some(key),
            _ => None,
        }
    }

    pub fn mv(&mut self, y: usize, x: usize) {
        write!(self.stdout, "{}", cursor::Goto(1 + x as u16, 1 + y as u16)).unwrap_or_else(|err| {
            warn!("Unable to mv cursor: {}", err);
        });
    }

    pub fn colour_on(&mut self, fg: usize, bg: usize) {
        let fgcol = match fg {
            0 => self.config.colour0,
            1 => self.config.colour1,
            2 => self.config.colour2,
            3 => self.config.colour3,
            4 => self.config.colour4,
            5 => self.config.colour5,
            6 => self.config.colour6,
            7 => self.config.colour7,
            8 => self.config.colourfg,
            _ => return (),
        };

        let bgcol = match bg {
            0 => self.config.colour0,
            1 => self.config.colour1,
            2 => self.config.colour2,
            3 => self.config.colour3,
            4 => self.config.colour4,
            5 => self.config.colour5,
            6 => self.config.colour6,
            7 => self.config.colour7,
            8 => self.config.colourbg,
            _ => return (),
        };

        write!(self.stdout, "{}{}", color::Fg(fgcol), color::Bg(bgcol)).unwrap_or_else(|err| {
            warn!("Unable to turn colour on: {}", err);
        });
    }

    pub fn colour_off(&mut self) {
        write!(
            self.stdout,
            "{}{}",
            color::Fg(self.config.colourfg),
            color::Bg(self.config.colourbg)
        )
        .unwrap_or_else(|err| {
            warn!("Unable to turn colour off: {}", err);
        });
    }

    pub fn colour_reset(&mut self) {
        write!(
            self.stdout,
            "{}{}",
            color::Fg(color::Reset),
            color::Bg(color::Reset)
        )
        .unwrap_or_else(|err| {
            warn!("Unable to turn colour off: {}", err);
        });
    }

    pub fn mvprintw(&mut self, y: usize, x: usize, text: &str) {
        write!(
            self.stdout,
            "{}{}",
            cursor::Goto(1 + x as u16, 1 + y as u16),
            text
        )
        .unwrap_or_else(|err| {
            warn!("Unable to mvprintw: {}", err);
        });
    }

    pub fn wrap_print(&mut self, y: usize, x: usize, width: usize, text: &str) {
        let len = text.len();
        let wid = width as usize - 3;
        let limit = if len > wid { wid } else { len };
        self.mvprintw(y, x, &text[..limit]);
        if len > wid {
            self.mvprintw(y, x + width - 3, "...");
        }
    }

    pub fn border(&mut self, lower_left: (usize, usize), dimensions: (usize, usize)) {
        let (y, x) = lower_left;
        let (height, width) = dimensions;

        self.mvprintw(y + 1 - height, x, self.config.ulcorner);
        self.mvprintw(y, x, self.config.llcorner);

        self.mvprintw(y + 1 - height, x + width - 1, self.config.urcorner);
        self.mvprintw(y, x + width - 1, self.config.lrcorner);

        for j in (y + 2 - height)..y {
            self.mvprintw(j, x, self.config.vline);
            self.mvprintw(j, x + width - 1, self.config.vline);
        }

        for i in (x + 1)..(x + width - 1) {
            self.mvprintw(y, i, self.config.hline);
            self.mvprintw(y + 1 - height, i, self.config.hline);
        }
    }

    pub fn rectangle(&mut self, ch: &str, lower_left: (usize, usize), dimensions: (usize, usize)) {
        let (y, x) = lower_left;
        let (height, width) = dimensions;

        for j in (y - height + 1)..y {
            for i in x..(x + width - 1) {
                self.mvprintw(j, i, ch);
                self.mvprintw(j, i + width - 1, ch);
            }
        }
    }

    pub fn clear(&mut self) {
        write!(self.stdout, "{}", clear::All).unwrap_or_else(|err| {
            warn!("Unable to clear stdout: {}", err);
        });
    }

    pub fn endwin(&mut self) {
        self.colour_reset();
        write!(
            self.stdout,
            "{}{}{}",
            clear::All,
            style::Reset,
            cursor::Goto(1, 1)
        )
        .unwrap_or_else(|err| {
            warn!("Unable to endwin: {}", err);
        });
    }
}
