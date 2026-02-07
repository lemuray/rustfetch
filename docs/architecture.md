# Understanding rustfetch's architecture

This file is reserved to **understanding the core structure of rustfetch** in a digestible and opinionated way, every change to the architecture must be documented.
<br>This file also contains the **reasons** this projects does things the way it does and its philosophy.

This project started as my **first Rust project**, so it has had many architecture iterations, but the current one is by far the **most idiomatic and structured**.

Lets break it down file by file:

<pre>
src/
├── <a href="#mainrs">main.rs</a>         # Entry point
├── <a href="#configrs">config.rs</a>       # TOML config
├── <a href="#clirs">cli.rs</a>          # CLI argument parsing
├── common/         # Common functions across OSes and files
│   ├── <a href="#displayrs">display.rs</a>  # Display formatting functions
│   └── <a href="#utilsrs">utils.rs</a>    # Shared utility functions
├── platform/       # OS-specific implementations
│   ├── <a href="#modrs-platform">mod.rs</a>      # Exposes modules based on OS
│   ├── linux.rs       
│   └── macos.rs        
└── sysinfo/        # Cross-platform system info
    └── <a href="#sharedrs">shared.rs</a>   # Generic sysinfo functions
tests/              # Tests for "cargo test"
└── <a href="#utils_testsrs">utils_tests.rs</a>  # Tests specific to utils

<a href="#installsh">install.sh</a>          # Bash installation script
</pre>

> Note: most mod.rs (and lib.rs) files have been omitted from this file tree as they all share the same purpose: exposing modules

## main.rs
Entry point for the program, **handles showing or not showing modules** based on current config settings and **skips arguments** if they are OS-bound and the target OS is different from the supported one:

- **Gets command line arguments** through [clap](https://docs.rs/clap/latest/clap/)
- **Creates a System variable** used for sysinfo functions
-  If the "--all" or "-a" flag is given, **skips config file parsing** entirely to enable all modules (This does not skip unsupported [platform specific modules](#platform)).
<br>Else, **gets config options** from [config.rs](#configrs) and shows them based on their boolean value.
- Gets the distro id from the dedicated function inside the **platform** crate
- Creates a **String vector** and adds the return values of the [display functions](#displayrs) to it if the config boolean for it is true and if the function returns Some(), if it returns None or the config for it is false it **just skips it**.
- Prints the info_lines vector alongside the logo's lines and **adds padding** to make all the lines be horizontally aligned. As the line get printed, they are colored according to the distro_id

## config.rs
Main file for configuration handling, **creates and parses a config file** or enables all features:

- Creates a **DisplayConfig** struct that includes all modules as boolean values
- **Creates two implementations** for DisplayConfig: **Default** for initial file creation and **All** where all modules are set to true
- **load_config()** -> returns a DisplayConfig struct from parsing the **config.toml**. If the file does not exist it will create it in the [default config directory](https://docs.rs/dirs/latest/dirs/fn.config_dir.html) and print a message.
- **load_all_config()** -> runs the set_all() function for DisplayConfig and returns its value.

## cli.rs
Uses [clap](https://docs.rs/clap/latest/clap/) to **parse command line arguments** and creates a public Cli struct with all the possible flags in it. This file also decides which description every flag should have when running "rustfetch --help".

Example of a standard flag:
![Example](./images/cli_argument_description.avif)

## display.rs
Contains all functions related to showing the values returned from other files as formatted text.

Also contains internal private functions such as **color_percentage()**, specific to display features.

## utils.rs
Contains **general purpose functions** shared across multiple files. These must undergo the highest level of [testing](#utils_testsrs) as they're used everywhere. 

## mod.rs (Platform)
Exposes modules based on operating system, for example:

- If the target OS is **Linux**, the "platform" module will use functions inside linux.rs
- If the target OS is **MacOS**, the "platform" module will use functions inside macos.rs

Note that **every function that's written in one file must be written in the other**, even if it will never run (As usual, the Rust compiler being strict)

## shared.rs
Contains all functions which run **regardless of OS**, the [sysinfo crate](https://docs.rs/sysinfo/latest/sysinfo/) is most used here.

Every function that uses sysinfo functions requires a **reference to the main System variable** that it will use internally.

Functions inside this file should never handle displaying the values, as that is almost always exclusively handled in [display.rs](#displayrs).

## utils_tests.rs
Testing file for utils functions, the testing requirements are usually:
- **Correct input** -> Should return the expected value
- **Edge cases** -> Used to **enhance error handling** and **understand how the function behaves** deeply. These should be **technically correct though very unlikely** inputs (E.g: rounding an f64::Infinity) and checking the output is still Infinity
- **Incorrect input** -> Testing the function with an **input thats incoherent with what the function is asking**, **assessing the function is returning an error** and, if present, checking if the **error message** is correct

## install.sh
Bash automated installation script for the program that can work through curl, its jobs are the following:
- Checks if the OS is supported, if it is not, it refuses to install
- Gets architecture and OS name and normalizes them to **match the release syntax**
- **Detects shell** (zsh, bash exc) to parse its config file, if it is not found it skips automatic shell integration
- Parses the config file to find any system info tools from a static list of the most famous CLIs similar to this (neofetch, fastfetch exc). If any is detected, it uses **regex** to assert the line is a standalone command and not nested in some complicated logic we shouldn't touch
- If any command is found and it is a **standalone**, it asks the user if they want to replace it with rustfetch. If there is no command at all it asks to append rustfetch. Else, it skips shell integration.
- **Fetches the latest version** from the normalized names we got at the beginning and installs it 

> Note: The file is not modified on-the-fly but its lines are assigned into an array and modified there, it then checks if the script modified any line and overwrites the whole file