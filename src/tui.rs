use log::{error, warn};
use std::io::{Stdin, Stdout, Write};
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, color, cursor, style};

const HORIZONTAL_LINE: &'static str = "─";
const VERTICAL_LINE: &'static str = "│";
const UPPER_LEFT_CORNER: &'static str = "┌";
const UPPER_RIGHT_CORNER: &'static str = "┐";
const LOWER_LEFT_CORNER: &'static str = "└";
const LOWER_RIGHT_CORNER: &'static str = "┘";

pub struct Window {
    stdin: Keys<Stdin>,
    stdout: RawTerminal<Stdout>,
}

impl Drop for Window {
    fn drop(&mut self) {
        self.endwin();
        self.show_cursor();
    }
}

impl Window {
    pub fn new(stdin: Stdin, stdout: Stdout) -> Result<Window, ()> {
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

    pub fn colour_on<F: color::Color, B: color::Color>(&mut self, fg: F, bg: B) {
        write!(self.stdout, "{}{}", color::Fg(fg), color::Bg(bg)).unwrap_or_else(|err| {
            warn!("Unable to turn colour on: {}", err);
        });
    }

    pub fn colour_off(&mut self) {
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

        self.mvprintw(y - height + 1, x, UPPER_LEFT_CORNER);
        self.mvprintw(y, x, LOWER_LEFT_CORNER);

        self.mvprintw(y - height + 1, x + width - 1, UPPER_RIGHT_CORNER);
        self.mvprintw(y, x + width - 1, LOWER_RIGHT_CORNER);

        for j in (y - height + 2)..y {
            self.mvprintw(j, x, VERTICAL_LINE);
            self.mvprintw(j, x + width - 1, VERTICAL_LINE);
        }

        for i in (x + 1)..(x + width - 1) {
            self.mvprintw(y, i, HORIZONTAL_LINE);
            self.mvprintw(y - height + 1, i, HORIZONTAL_LINE);
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
