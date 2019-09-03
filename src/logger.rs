/// Logging functionality.

/// Dispatch logger to report errors and other information.
pub fn setup_logger() {
    let dispatcher = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.line().unwrap_or_else(|| 0),
                record.level(),
                message
            ))
        })
        .chain(std::io::stderr());

    // Print fancy title
    eprintln!("\n\n\n                __ ");
    eprintln!("   __  ______ _/ /_");
    eprintln!("  / / / / __ `/ __/");
    eprintln!(" / /_/ / /_/ / /_  ");
    eprintln!(" \\__, /\\__,_/\\__/  ");
    eprintln!("/____/             \n\n");
    eprintln!("Log:\n");

    match dispatcher.apply() {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Logger already called: unable to dispatch!");
            ()
        }
    };
}
