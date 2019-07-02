use std::env;
use std::process;
use yat::{logger::setup_logger, look_for_save, View};

fn main() {
    // Set up loggin to stderr
    setup_logger();

    // Check for existence of valid save file
    let view_result = match look_for_save(env::args()) {
        Ok(filename) => View::new_from_save(filename),
        Err(_) => View::new(),
    };

    // Create UI
    let mut view = view_result.unwrap_or_else(|_| {
        process::exit(1);
    });

    // Run todo list manager
    view.run();
}
