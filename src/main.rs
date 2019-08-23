use std::env;
use std::process;
use yat::{
    config::{check_for_config, Config},
    logger::setup_logger,
    look_for_save, View,
};

fn main() {
    // Set up loggin to stderr
    setup_logger();

    // Configuration
    let mut config = Config::default();
    let found_config = check_for_config();
    match &found_config {
        Some(configbuf) => {
            config = configbuf.config(config);
        }
        None => (),
    }

    // Check for existence of valid save file
    let view_result = match look_for_save(env::args()) {
        Ok(filename) => View::new_from_save(filename, config),
        Err(_) => View::new(config),
    };

    // Create UI
    let mut view = view_result.unwrap_or_else(|_| {
        process::exit(1);
    });

    // Run todo list manager
    view.run();
}
