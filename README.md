# *yat* -- yet another todo-list
A simple terminal todo-list manager written in Rust. The terminal user interface (TUI) is built around the excellent [termion](https://crates.io/crates/termion) crate.

**Disclaimer**: this is a work in progress! When it reaches a usable state hopefully it will be published on crates.io.

## Installation
Requires an installation of [Rust](https://www.rust-lang.org/tools/install). Recommended build profile is release:

    cargo build --release

## Usage
Can be run with cargo from the root of the directory. Logging is provided by the nifty [fern](https://crates.io/crates/fern) and [log](https://crates.io/crates/log) crates; this will print to stderr, but these will be missed behind the TUI, so it might be useful to redirect them to a file:

    cargo run --release 2>err.log

Once running, **yat** uses the following (vim-*ish*) key bindings:

|Key      | Command                     |
|---------|-----------------------------|
|a        | add new task                |
|e        | edit selected task          |
|d        | delete selected task        |
|w        | save todo list to file      |
|q        | quit                        |
|k, Up    | move selection up           |
|j, Down  | move selection down         |
|l, Enter | focus on selected sub-task  |
|h, b     | return focus to parent task |
|>        | increase task priority      |
|<        | decrease task priority      |

The user interface shows 4 panels: parent task, tasks, sub-tasks and selection. The tasks panel is the main panel, which allows you to navigate between tasks.

Usually **yat** will save to $HOME/.todo/save.txt, which will be created the first time it runs. You can specify a custom file to load (or create) by passing it as a first argument on the command line.

## To Do
1. Loading: although loading from a save file is implemented, the parsing functionality can be made more robust.
2. Configuration: user configuration e.g. custom colour scheme, keybindings etc.
3. Clean-up: general code clean-up and refactoring, including more extensive commenting.
