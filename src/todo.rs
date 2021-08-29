/// Functionality for storing todo lists in a tree data structure.

use log::{info, warn};
use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::rc::{Rc, Weak};

/// Task priority.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
}

/// Node in the todo list tree structure.
#[derive(Debug, Clone)]
pub struct ToDo {
    pub task: String,
    pub complete: bool,
    pub priority: Option<Priority>,
    pub parent: Weak<RefCell<ToDo>>,
    pub sub_tasks: Vec<Rc<RefCell<ToDo>>>,
}

impl ToDo {
    /// Create new todo list tree structure.
    pub fn new(task: &str, parent: Weak<RefCell<ToDo>>) -> ToDo {
        let sub_tasks: Vec<Rc<RefCell<ToDo>>> = Vec::new();
        ToDo {
            task: String::from(task),
            complete: false,
            priority: None,
            parent,
            sub_tasks,
        }
    }

    /// Find the task hierachy.
    pub fn task_path(&self, path: &mut String) {
        if let Some(parent_todo) = self.parent.upgrade() {
            let previous = format!("{}: ", &parent_todo.borrow().task);
            path.insert_str(0, &previous);
            parent_todo.borrow().task_path(path);
        }
        ()
    }

    /// Convert todo list node to string format.
    fn to_string(&self) -> String {
        let mut output = String::new();

        match self.complete {
            true => {
                output.push_str("[X] ");
                ()
            }
            false => {
                output.push_str("[ ] ");
                ()
            }
        }

        match self.priority {
            Some(Priority::Low) => {
                output.push_str("(C) ");
                ()
            }
            Some(Priority::Medium) => {
                output.push_str("(B) ");
                ()
            }
            Some(Priority::High) => {
                output.push_str("(A) ");
                ()
            }
            None => {
                output.push_str("( ) ");
                ()
            }
        }

        output.push_str(&self.task);
        output.push('\n');
        output
    }

    /// Convert all sub-tasks to string format.
    fn all_to_string(&self, tabs: usize, buf: &mut String) {
        for sub_task_rc in self.sub_tasks.iter() {
            let sub_task = sub_task_rc.borrow();
            let mut sub_task_str = sub_task.to_string();
            let tab_pad = "    ".repeat(tabs);
            sub_task_str.insert_str(0, &tab_pad);
            buf.push_str(&sub_task_str);
            sub_task.all_to_string(tabs + 1, buf);
        }
    }

    /// Save todo list tree in string format to text file.
    fn save_current(&self, filename: &Path) {
        let mut buffer = String::new();
        self.all_to_string(0, &mut buffer);

        let mut file = match File::create(filename) {
            Ok(f) => f,
            Err(err) => {
                warn!("Unable to open file to save: {}.", err);
                return;
            }
        };

        match file.write_all(buffer.as_bytes()) {
            Ok(_) => (),
            Err(err) => {
                warn!("Unable to write to save file: {}", err);
                return;
            }
        };
        info!("Todo list saved to file.");
    }

    /// Traverse tree back to root node and save.
    pub fn save(&self, filename: &Path) {
        if let Some(parent_todo) = self.parent.upgrade() {
            parent_todo.borrow().save(filename)
        } else {
            self.save_current(filename)
        }
    }

    /// Convert from string format into ToDo node.
    pub fn from_string(text: &str, parent: Weak<RefCell<ToDo>>) -> ToDo {
        let complete = match text.chars().nth(1) {
            Some(ch) => {
                if ch == 'X' {
                    true
                } else {
                    false
                }
            }
            None => false,
        };

        let priority = match text.chars().nth(5) {
            Some(ch) => match ch {
                'A' => Some(Priority::High),
                'B' => Some(Priority::Medium),
                'C' => Some(Priority::Low),
                _ => None,
            },
            None => None,
        };

        let mut todo = Self::new(&text[8..], parent);
        todo.complete = complete;
        todo.priority = priority;
        todo
    }

    /// Reorder subtasks based on priority
    pub fn sort_by_priority(&mut self) {
        self.sub_tasks.sort_by(|a, b| {
            // Reverse order so we treat None properly
            b.borrow().priority.cmp(&a.borrow().priority)
        });
    }
}
