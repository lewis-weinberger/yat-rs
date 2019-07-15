# *yat* -- yet another todo-list
>A simple terminal todo-list manager written in Rust. 

The terminal user interface (TUI) is built around the excellent [termion](https://crates.io/crates/termion) crate.

**Table of contents**
1. [Installation](#installation)
2. [Usage](#usage)
3. [Customisation](#customisation)
4. [To Do](#to-do)
5. [License](#license)

**Disclaimer**: this is a work in progress! When it reaches a usable state hopefully it will be published on crates.io.

<a name="installation"></a>
## Installation
Requires an installation of [Rust](https://www.rust-lang.org/tools/install). Recommended build profile is release:
    
    $ curl https://sh.rustup.rs -sSf | sh         # install Rust
    $ git clone https://github.com/drvog/yat-rs   # clone repository
    $ cd yat-rs                                   # change into source directory
    $ cargo run --release                         # compile and run

<a name="usage"></a>
## Usage
Can be run with cargo from the root of the directory. Logging is provided by the nifty [fern](https://crates.io/crates/fern) and [log](https://crates.io/crates/log) crates; this will print to stderr, but these will be missed behind the TUI, so it might be useful to redirect them to a file:

    $ cargo run --release 2>err.log

Once running, **yat** uses the following default key bindings:

|Key      | Command                     |
|---------|-----------------------------|
|a        | add new task                |
|e        | edit selected task          |
|d        | delete selected task        |
|u        | move selected task up       |
|n        | move selected task down     |
|w        | save todo list to file      |
|q        | quit                        |
|k, Up    | move selection up           |
|j, Down  | move selection down         |
|l, Enter | focus on selected sub-task  |
|h, b     | return focus to parent task |
|>        | increase task priority      |
|<        | decrease task priority      |

The user interface shows 4 panels: parent task, tasks, sub-tasks and selection. The tasks panel is the main panel, which allows you to navigate between tasks.

![Screenshot](screenshot.png)

Moving the selection can be done with the UP and DOWN keys or alternatively with the vi(m) keys:

      k
      |
    h─.─l
      |
      j

The layout of the task on the panel is as follows:

    > [ ] todo
    │  │   │         
    │  │   └─ this is the content of the task (colour indicates priority).
    │  │
    │  └─ this shows task completion: [X] = completed, [ ] = not completed.
    │
    └─ this indicates that this task is currently selected.

Usually **yat** will save to $HOME/.todo/save.txt, which will be created the first time it runs. You can specify a custom file to load (or create) by passing it as a first argument on the command line. The formatting of the save file is as follows:

    [ ] ( ) todo
     │   │   │         
     │   │   └─ this is the content of the task.
     │   │
     │   └─ this shows task priority: ( ) = no priority, (C) = low priority,
     │      (B) = medium priority, (A) = high priority.
     │
     └─ this shows task completion: [X] = completed, [ ] = not completed. 

<a name="customisation"></a>
## Customisation
It is possible to tweak the appearance of **yat** at runtime using a configuration file, as by default it only uses the basic ANSI colours of your terminal [emulator]. **yat** will look for configuration at `~/.todo/config.toml`, which allows you to change the panel borders and the colour-scheme. The format for `config.toml` is:

    # ~/.todo/config.toml
    [borders]                   # Panel customisation
    hline = "─"                 # horizontal line
    vline = "│"                 # vertical line
    ulcorner = "┌"              # upper left corner
    urcorner = "┐"              # upper right corner
    llcorner = "└"              # lower left corner
    lrcorner = "┘"              # lower right corner

    [colours]                   # Colourscheme customisation
    colour0 = [88, 110, 117]    # black
    colour1 = [220, 50, 47]     # red
    colour2 = [133, 153, 0]     # green 
    colour3 = [181, 137, 0]     # yellow
    colour4 = [38, 139, 210]    # blue
    colour5 = [211, 54, 130]    # magenta
    colour6 = [42, 161, 152]    # cyan
    colour7 = [7, 54, 66]       # white
    colourfg = [131, 148, 150]  # foreground
    colourbg = [0, 43, 54]      # background

Currently **yat** requires all of these to be specified otherwise it will ignore them and use the default configuration. The `borders` must be valid unicode, and the `colours` are specified as (r, g, b) where r/g/b are u8 integers, i.e. values in the interval (0, 256]. Note importantly this will only work if your terminal supports 24-bit colours ("True Color"), and is untested on incompatible terminal emulators. Some examples are provided in the [configs](configs) directory.

<a name="to-do"></a>
## To Do
1. Loading: although loading from a save file is implemented, the parsing functionality should be made more robust.
2. Configuration: user configuration of keybindings, and allowing the user to specify partial configuration.
3. Clean-up: general code clean-up and refactoring, including more extensive commenting.
4. Windows: currently **yat** is built on top of termion, which works on UNIX-like terminals, and therefore lacks Windows support.

Contributions welcome! Please submit an issue or pull request.

<a name="license"></a>
## License

[MIT License](LICENSE)
