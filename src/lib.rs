pub mod config;
pub mod logger;
mod todo;
mod tui;

use dirs::home_dir;
use log::{info, warn};
use std::cell::RefCell;
use std::env::Args;
use std::fs::{create_dir, metadata, File};
use std::io::{self, Read};
use std::path::PathBuf;
use std::rc::{Rc, Weak};
use std::str::Lines;
use termion::event::Key;
use todo::{Priority, ToDo};
use tui::Window;

pub fn look_for_save(mut args: Args) -> Result<PathBuf, ()> {
    args.next();

    match args.next() {
        Some(arg) => {
            let filename = PathBuf::from(&arg);
            match metadata(&filename) {
                Ok(_) => return Ok(filename),
                Err(err) => {
                    warn!("Provided save file does not exist: {}", err);
                    Err(())
                }
            }
        }
        None => {
            let mut filename = match home_dir() {
                Some(dir) => dir,
                None => {
                    warn!("Unable to find home directory.");
                    return Err(());
                }
            };
            filename.push(".todo");

            match metadata(&filename) {
                Ok(_) => {
                    filename.push("save.txt");
                    match metadata(&filename) {
                        Ok(_) => {
                            info!("Found save file.");
                            Ok(filename.to_path_buf())
                        }
                        Err(err) => {
                            warn!("$HOME/.todo/save.txt does not exist: {}", err);
                            Err(())
                        }
                    }
                }
                Err(_) => {
                    create_dir(filename).unwrap_or_else(|err| {
                        warn!("Unable to create directory ~/.todo: {}", err);
                    });
                    info!("Created $HOME/.todo directory.");
                    Err(())
                }
            }
        }
    }
}

pub struct View<'a> {
    window: Window<'a>,
    current_task: Rc<RefCell<ToDo>>,
    selection: Option<usize>,
    root: bool,
    quit: bool,
    save_file: Option<PathBuf>,
}

impl<'a> View<'a> {
    pub fn new(config: config::Config<'a>) -> Result<View<'a>, ()> {
        let root = ToDo::new("", Weak::new());
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut window = Window::new(stdin, stdout, config)?;
        window.colour_off();

        info!("Created new View.");
        Ok(View {
            window,
            current_task: Rc::new(RefCell::new(root)),
            selection: None,
            root: true,
            quit: false,
            save_file: None,
        })
    }

    pub fn new_from_save(filename: PathBuf, config: config::Config<'a>) -> Result<View<'a>, ()> {
        let root = ToDo::new("", Weak::new());
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut window = Window::new(stdin, stdout, config)?;
        window.colour_off();

        let mut view = View {
            window,
            current_task: Rc::new(RefCell::new(root)),
            selection: None,
            root: true,
            quit: false,
            save_file: Some(filename.clone()),
        };

        let proot = Rc::clone(&view.current_task);
        if let Ok(buf) = Self::load(filename) {
            match view.fill_children(&mut buf.lines(), 0) {
                Ok(()) => {
                    view.current_task = proot;
                }
                Err(err) => {
                    warn!("Unable to parse save file: {}", err);
                    let new_root = ToDo::new("", Weak::new());
                    view.current_task = Rc::new(RefCell::new(new_root));
                }
            }
        };

        info!("Created new View from save file.");
        Ok(view)
    }

    fn load(filename: PathBuf) -> Result<String, ()> {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                warn!("Unable to load file.");
                return Err(());
            }
        };

        let mut buffer = String::new();
        match file.read_to_string(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(_) => {
                warn!("Unable to read from save file.");
                return Err(());
            }
        }
    }

    fn fill_children(&mut self, buf: &mut Lines, tabs: usize) -> Result<(), &'static str> {
        // Parse save file line by line
        if let Some(line) = buf.next() {
            // Use indentation to determine where to insert each task. If
            // indentation is the same as the previous line then we continue
            // adding sub-tasks to the current line.
            let num_tabs = tab_num(&line);
            let current = Rc::clone(&self.current_task);
            if num_tabs == tabs + 1 {
                // If indentation is increased compared to the previous line,
                // then the previously added sub-task is the new current task
                let n = self.current_task.borrow().sub_tasks.len();
                if n == 0 {
                    return Err("Can't have child without parent.");
                }
                let new_current = &current.borrow().sub_tasks[n - 1];
                self.current_task = Rc::clone(&new_current);
            } else if num_tabs < tabs {
                // If indentation is decreased compared to the previous line,
                // then the parent (or an even earlier ancestor) of the
                // previous task is the new current task
                self.ancestor(tabs - num_tabs);
            } else if num_tabs > tabs + 1 {
                return Err("Too much indentation.");
            }

            self.add_task_from_string(line.trim_start());

            // Continue onto next line
            self.fill_children(buf, num_tabs)?;
        }
        Ok(())
    }

    fn ancestor(&mut self, level: usize) {
        let current = Rc::clone(&self.current_task);
        let pparent = &current.borrow().parent;
        if level > 0 {
            if let Some(parent) = pparent.upgrade() {
                self.current_task = Rc::clone(&parent);
                self.ancestor(level - 1);
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            self.list_tasks();
            match self.window.getch() {
                Some(key) if key == self.window.config.quit => {
                    self.quit = true;
                }
                Some(key) if key == self.window.config.back => match self.root {
                    true => (),
                    false => break,
                },
                Some(key) if key == self.window.config.save => self.save(),
                Some(key) if key == self.window.config.add => self.add_task_from_input(),
                Some(key) if key == self.window.config.edit => self.edit_task(),
                Some(key) if key == self.window.config.delete => self.remove_task(),
                Some(key) if key == self.window.config.task_up => self.move_task(true),
                Some(key) if key == self.window.config.task_down => self.move_task(false),
                Some(key) if key == self.window.config.focus => self.new_focus(),
                Some(key) if key == self.window.config.complete => self.complete_task(),
                Some(key) if key == self.window.config.up => self.move_selection(true),
                Some(key) if key == self.window.config.down => self.move_selection(false),
                Some(key) if key == self.window.config.increase => self.increase_priority(),
                Some(key) if key == self.window.config.decrease => self.decrease_priority(),
                Some(_) => (),
                None => (),
            }
            if self.quit {
                self.window.endwin();
                break;
            }
        }
    }

    fn input_dialogue(&mut self, prompt: &str) -> String {
        self.dialogue(prompt, "")
    }

    fn edit_dialogue(&mut self, prompt: &str, index: usize) -> String {
        let mut original = String::new();
        {
            let sub_tasks = &self.current_task.borrow().sub_tasks;
            original.push_str(&sub_tasks[index].borrow().task);
        }
        self.dialogue(prompt, &original)
    }

    fn dialogue(&mut self, prompt: &str, text: &str) -> String {
        let (ymax, xmax) = self.window.get_max_yx();
        self.window.border((ymax - 1, 0), (3, xmax));
        self.window
            .rectangle(&(' '.to_string())[..], (ymax - 1, 1), (2, xmax - 2));
        self.window.colour_on(0, 7);
        self.window.mvprintw(ymax - 2, 2, prompt);
        self.window.colour_off();
        self.window.mvprintw(ymax - 2, 3 + prompt.len(), text);
        self.window.refresh();
        self.window.show_cursor();

        let mut entry = String::from(text);
        let mut index = entry.len();
        loop {
            match self.window.getch() {
                Some(Key::Char('\n')) => break,
                Some(Key::Char(ch)) => {
                    if index >= entry.len() {
                        entry.push(ch);
                    } else {
                        entry.insert(index, ch);
                    }
                    index += 1;
                    ()
                }
                Some(Key::Backspace) => {
                    if entry.len() > 0 {
                        self.window
                            .mvprintw(ymax - 2, 3 + (prompt.len() + entry.len() - 1), " ");
                        entry.remove(index - 1);
                        index -= 1;
                    }
                    ()
                }
                Some(Key::Delete) => {
                    if entry.len() > 0 && index < entry.len() {
                        self.window
                            .mvprintw(ymax - 2, 3 + (prompt.len() + entry.len() - 1), " ");
                        entry.remove(index);
                    }
                    ()
                }
                Some(Key::Left) => {
                    if index > 0 {
                        index -= 1;
                    }
                    ()
                }
                Some(Key::Right) => {
                    if index < entry.len() {
                        index += 1;
                    }
                    ()
                }
                _ => (),
            }
            self.window.mvprintw(ymax - 2, 3 + prompt.len(), &entry);
            self.window
                .mv(ymax - 2, 3 + (prompt.len() + &entry[0..index].len()));
            self.window.refresh();
        }
        entry
    }

    fn list_tasks(&mut self) {
        self.window.clear();
        self.window.hide_cursor();

        let (ymax, xmax) = self.window.get_max_yx();

        // Panels
        let mut path = self.current_task.borrow().task.clone();
        self.current_task.borrow().task_path(&mut path);
        self.window.mvprintw(1, 1, &path);
        self.window.border((2, 0), (3, xmax));
        self.window.border((ymax - 4, 0), (ymax - 6, xmax / 2));
        self.window
            .border((ymax - 4, xmax / 2), (ymax - 6, xmax / 2));
        self.window.border((ymax - 1, 0), (3, xmax));

        self.window.colour_on(4, 8);
        self.window.mvprintw(0, 2, "Parent");
        self.window.mvprintw(3, 2, "Tasks");
        self.window.mvprintw(3, xmax / 2 + 2, "Sub-tasks");
        self.window.mvprintw(ymax - 3, 2, "Selection");
        self.window.colour_off();

        self.window.colour_on(6, 8);
        match self.selection {
            Some(index) => {
                if index > self.current_task.borrow().sub_tasks.len() - 1 {
                    warn!("Index larger than it should be.");
                    self.selection = None;
                } else {
                    self.window.mvprintw(4 + index, 1, ">");
                    self.window.mvprintw(
                        ymax - 2,
                        2,
                        &self.current_task.borrow().sub_tasks[index].borrow().task,
                    );
                }
            }
            None => (),
        };
        self.window.colour_off();

        let sub_tasks = &self.current_task.borrow().sub_tasks;
        let mut y = 4;
        for (i, elem) in sub_tasks.iter().enumerate() {
            if elem.borrow().complete {
                self.window.mvprintw(y, 3, "[");
                self.window.colour_on(4, 8);
                self.window.mvprintw(y, 4, "X");
                self.window.colour_off();
                self.window.mvprintw(y, 5, "]");
            } else {
                self.window.mvprintw(y, 3, "[ ]");
            }
            match elem.borrow().priority {
                Some(Priority::Low) => {
                    self.window.colour_on(2, 8);
                }
                Some(Priority::Medium) => {
                    self.window.colour_on(3, 8);
                }
                Some(Priority::High) => {
                    self.window.colour_on(1, 8);
                }
                _ => (),
            };
            self.window
                .wrap_print(y, 7, xmax / 2 - 8, &format!("{}", elem.borrow().task));
            self.window.colour_off();
            y += 1;

            if let Some(index) = self.selection {
                if index == i {
                    let mut yy = 4;
                    for sub_elem in elem.borrow().sub_tasks.iter() {
                        if sub_elem.borrow().complete {
                            self.window.mvprintw(yy, xmax / 2 + 3, "[");
                            self.window.colour_on(4, 8);
                            self.window.mvprintw(yy, xmax / 2 + 4, "X");
                            self.window.colour_off();
                            self.window.mvprintw(yy, xmax / 2 + 5, "]");
                        } else {
                            self.window.mvprintw(yy, xmax / 2 + 3, "[ ]");
                        }
                        match sub_elem.borrow().priority {
                            Some(Priority::Low) => {
                                self.window.colour_on(2, 8);
                            }
                            Some(Priority::Medium) => {
                                self.window.colour_on(3, 8);
                            }
                            Some(Priority::High) => {
                                self.window.colour_on(1, 8);
                            }
                            _ => (),
                        };
                        self.window.wrap_print(
                            yy,
                            xmax / 2 + 7,
                            xmax / 2 - 8,
                            &format!("{}", sub_elem.borrow().task),
                        );
                        self.window.colour_off();
                        yy += 1;
                    }
                }
            };
        }
        self.window.refresh();
    }

    fn increase_priority(&mut self) {
        match self.selection {
            Some(index) => {
                let current = self.current_task.borrow();
                let mut sub_task = current.sub_tasks[index].borrow_mut();
                sub_task.priority = match sub_task.priority {
                    None => Some(Priority::Low),
                    Some(Priority::Low) => Some(Priority::Medium),
                    Some(Priority::Medium) => Some(Priority::High),
                    Some(Priority::High) => Some(Priority::High),
                };
            }
            None => (),
        }
    }

    fn decrease_priority(&mut self) {
        match self.selection {
            Some(index) => {
                let current = self.current_task.borrow();
                let mut sub_task = current.sub_tasks[index].borrow_mut();
                sub_task.priority = match sub_task.priority {
                    None => None,
                    Some(Priority::Low) => None,
                    Some(Priority::Medium) => Some(Priority::Low),
                    Some(Priority::High) => Some(Priority::Medium),
                };
            }
            None => (),
        }
    }

    fn add_task_from_input(&mut self) {
        let task = self.input_dialogue("New Task:");
        let parent = Rc::downgrade(&self.current_task);
        let todo = ToDo::new(&task, parent);
        let sub_tasks = &mut self.current_task.borrow_mut().sub_tasks;
        sub_tasks.push(Rc::new(RefCell::new(todo)));
        self.selection = Some(sub_tasks.len() - 1);
    }

    fn add_task_from_string(&mut self, input: &str) {
        let parent = Rc::downgrade(&self.current_task);
        let todo = ToDo::from_string(input, parent);
        let sub_tasks = &mut self.current_task.borrow_mut().sub_tasks;
        sub_tasks.push(Rc::new(RefCell::new(todo)));
        self.selection = Some(sub_tasks.len() - 1);
    }

    fn complete_task(&mut self) {
        let sub_tasks = &mut self.current_task.borrow_mut().sub_tasks;
        match self.selection {
            Some(index) => {
                let mut sub_task = sub_tasks[index].borrow_mut();
                sub_task.complete = !sub_task.complete;
            }
            None => (),
        }
    }

    fn move_task(&mut self, up: bool) {
        let sub_tasks = &mut self.current_task.borrow_mut().sub_tasks;
        if let Some(index) = self.selection {
            if up {
                let new_index = if index == 0 {
                    sub_tasks.len() - 1
                } else {
                    index - 1
                };
                sub_tasks.swap(new_index, index);
                self.selection = Some(new_index);
            } else {
                let new_index = if index == sub_tasks.len() - 1 {
                    0
                } else {
                    index + 1
                };
                sub_tasks.swap(new_index, index);
                self.selection = Some(new_index);
            }
        }
    }

    fn new_focus(&mut self) {
        let previous_root = self.root.clone();
        let previous_selection = self.selection.clone();
        let psub_tasks = Rc::clone(&self.current_task);
        let sub_tasks = &psub_tasks.borrow().sub_tasks;
        match self.selection {
            Some(index) => {
                // Focus on sub-task
                let sub_task = &sub_tasks[index];
                self.current_task = Rc::clone(sub_task);
                self.root = false;
                self.selection = if self.current_task.borrow().sub_tasks.len() > 0 {
                    Some(0)
                } else {
                    None
                };
                self.run();

                // Return to parent task (unwrap cannot panic here)
                self.current_task = sub_task.borrow().parent.upgrade().unwrap();
                self.root = previous_root;
                self.selection = previous_selection;
            }
            None => (),
        }
    }

    fn edit_task(&mut self) {
        match self.selection {
            Some(index) => {
                let task = self.edit_dialogue("Edit Task:", index);
                let current_task = self.current_task.borrow_mut();
                let mut sub_task = current_task.sub_tasks[index].borrow_mut();
                sub_task.task = task.to_string();
            }
            None => (),
        }
    }

    fn move_selection(&mut self, ifup: bool) {
        self.selection = match self.selection {
            Some(index) => {
                if ifup {
                    self.up(index)
                } else {
                    self.down(index)
                }
            }
            None => match self.current_task.borrow().sub_tasks.len() {
                0 => None,
                _ => Some(0),
            },
        };
    }

    fn up(&self, index: usize) -> Option<usize> {
        let ntasks = self.current_task.borrow().sub_tasks.len();
        if index as isize - 1 < 0 {
            Some(index + ntasks - 1)
        } else {
            Some(index - 1)
        }
    }

    fn down(&self, index: usize) -> Option<usize> {
        let ntasks = self.current_task.borrow().sub_tasks.len();
        if index + 1 >= ntasks {
            Some(index + 1 - ntasks)
        } else {
            Some(index + 1)
        }
    }

    fn popup(&mut self, prompt: &str) -> bool {
        let (ymax, xmax) = self.window.get_max_yx();
        self.window.border((ymax - 1, 0), (3, xmax));
        self.window
            .rectangle(&(' '.to_string())[..], (ymax - 1, 1), (2, xmax - 2));
        self.window.colour_on(1, 7);
        self.window.mvprintw(ymax - 2, 2, prompt);
        self.window.colour_off();
        self.window.refresh();

        let mut choice = false;
        loop {
            match self.window.getch() {
                Some(Key::Char('y')) => {
                    choice = true;
                    break;
                }
                Some(Key::Char('n')) => break,
                Some(Key::Char('q')) => break,
                Some(Key::Char('b')) => break,
                _ => (),
            }
        }
        choice
    }

    fn remove_task(&mut self) {
        match self.selection {
            Some(index) => {
                if self.popup("Are you sure you want to delete this task? y/n") {
                    let mut current_task = self.current_task.borrow_mut();
                    current_task.sub_tasks.remove(index);
                    self.selection = None;
                }
            }
            None => (),
        }
    }

    fn save(&self) {
        let current = self.current_task.borrow();
        let filename = match self.save_file.clone() {
            Some(f) => f,
            None => {
                let mut buffer = match home_dir() {
                    Some(dir) => dir,
                    None => {
                        warn!("Unable to locate home directory.");
                        return;
                    }
                };
                buffer.push(".todo/save.txt");
                buffer
            }
        };

        current.save(filename.as_path())
    }
}

fn tab_num(line: &str) -> usize {
    let mut num = 0;
    while line[num..].starts_with(" ") {
        num += 1;
    }
    num / 4
}
