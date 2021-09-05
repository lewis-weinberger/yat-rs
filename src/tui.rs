/// Terminal user interface (TUI) functionality, with ncurses-like API,
/// built on top of the termion crate.
use crate::config::Config;
use log::{error, warn};
use std::io::{Stdin, Stdout, Write};
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, color, cursor, style};
use unicode_width::UnicodeWidthStr;

/// A wrapper around the terminal for creating a window.
pub struct Window<'a> {
    /// Key input from Stdin.
    stdin: Keys<Stdin>,
    /// Stdout, with terminal in raw-mode (no input line buffering, no echo).
    stdout: RawTerminal<Stdout>,
    /// Yat configuration.
    pub config: Config<'a>,
}

impl<'a> Drop for Window<'a> {
    /// Ensure the terminal is reset if the Window is dropped.
    fn drop(&mut self) {
        self.endwin();
        self.show_cursor();
    }
}

impl<'a> Window<'a> {
    /// Create a new Window, using terminal's stdin and stdout.
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

    /// Find the terminal's dimensions.
    pub fn get_max_yx(&self) -> (usize, usize) {
        let (y, x) = termion::terminal_size().unwrap_or_else(|err| {
            warn!("Unable to determine terminal size: {}.", err);
            (0, 0)
        });
        (x as usize, y as usize)
    }

    /// Hide cursor from terminal.
    pub fn hide_cursor(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap_or_else(|err| {
            warn!("Unable to hide cursor: {}.", err);
        });
    }

    /// Display cursor on terminal.
    pub fn show_cursor(&mut self) {
        write!(self.stdout, "{}", cursor::Show).unwrap_or_else(|err| {
            warn!("Unable to show cursor: {}", err);
        });
    }

    /// Flush stdout buffer to terminal.
    pub fn refresh(&mut self) {
        self.stdout.flush().unwrap_or_else(|err| {
            warn!("Unable to flush stdout: {}", err);
        });
    }

    /// Return the key input from stdin.
    pub fn getch(&mut self) -> Option<Key> {
        match self.stdin.next() {
            Some(Ok(key)) => Some(key),
            _ => None,
        }
    }

    /// Move the cursor to position at row y, column x (zero-indexed).
    pub fn mv(&mut self, y: usize, x: usize) {
        write!(self.stdout, "{}", cursor::Goto(1 + x as u16, 1 + y as u16)).unwrap_or_else(|err| {
            warn!("Unable to mv cursor: {}", err);
        });
    }

    /// Add colour to subsequent printed text.
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
            _ => return,
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
            _ => return,
        };

        write!(self.stdout, "{}{}", color::Fg(fgcol), color::Bg(bgcol)).unwrap_or_else(|err| {
            warn!("Unable to turn colour on: {}", err);
        });
    }

    /// Reset colours to default foreground and background.
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

    /// Reset colours to terminal defaults.
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

    /// Print text at row y, column x (zero-indexed).
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

    /// Print text at row y, column x (zero-indexed), truncated to ensure
    /// the text does not spill beyond width.
    pub fn wrap_print(&mut self, y: usize, x: usize, width: usize, text: &str) {
        let len = UnicodeWidthStr::width(text); // displayed width
        let mut end = text.len();
        if len > width - 3 {
            self.mvprintw(y, x + width - 3, "...");
            let mut n = (len - (width - 3)) as isize;
            let mut m = end;
            while n > 0 {
                end -= 1;
                if text.is_char_boundary(end) {
                    n -= UnicodeWidthStr::width(&text[end..m]) as isize;
                    m = end;
                }
            }
        }
        self.mvprintw(y, x, &text[..end]);
    }

    /// Print a rectangular border.
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

    /// Fill a rectangular region with character ch.
    pub fn rectangle(&mut self, ch: char, lower_left: (usize, usize), dimensions: (usize, usize)) {
        let (y, x) = lower_left;
        let (height, width) = dimensions;
        let mut buf = [0; 4];
        let c = ch.encode_utf8(&mut buf);

        for j in (y - height + 1)..(y + 1) {
            for i in x..(x + width) {
                self.mvprintw(j, i, c);
            }
        }
    }

    /// Clear stdout.
    pub fn clear(&mut self) {
        write!(self.stdout, "{}", clear::All).unwrap_or_else(|err| {
            warn!("Unable to clear stdout: {}", err);
        });
    }

    /// Reset stdout.
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
