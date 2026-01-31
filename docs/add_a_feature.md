# Planning on adding a feature?
First of all, if you haven't done it yet, **read the [contributing rules](../CONTRIBUTING.md)**. Now lets get specific on what you want to add.



## Adding a module
Modules are the **essential part of this CLI** and they are each field that is shown once you run "rustfetch".

### Example of a module:
```
CPU: AMD Ryzen 5 5600X
```
First, to determine how you're going to add said feature read the following list of possible ways you can get a concrete way of retrieving specific informations:

- **Generic system info** (E.g: CPU Name, Uptime exc) -> look into [sysinfo docs](https://docs.rs/sysinfo/latest/sysinfo/index.html).
These go into ```src/sysinfo/shared.rs```

- **Specific OS-bound info** (E.g: Init system on linux) -> these informations will probably be in a **file on the target system**, in this case ```/run/systemd/system```.
These go into ```src/platform/OS_NAME.rs```

- **Other** -> **If what you're trying to implement isn't listed here**, search for the specific crate you're going to use (if any) and **follow existing patterns**. In your PR description, include why you thought this didn't match any other known pattern listed above

Every module is part of the ```config.toml``` file, so read [how to add a config option](#adding-config-related-features).

Always remember to read and apply the [tests](#tests) section.



## Adding a helper function
To add a helper function, **minimize the use of heavy crates** and:
<br> If they are **small and specific** to the feature you're adding they can be kept as a **private function** inside the file itself.
<br> If they get **too big** or can be **shared across multiple files**

These must go into in ```src/common/utils.rs``` and have **doc comments** explaining their features thoroughly.

Always remember to read and apply the [tests](#tests) section.

### Example of a good helper function:
```
/// Returns true if the value is even and false if the value is odd
/// Is used to determine ...
pub fn is_even(input_value: i32) -> bool {
    input_value % 2 == 0
}
```


## Adding a CLI option
Lets suppose you want to add a flag for the rustfetch command, first get familiar with the [clap crate](https://docs.rs/clap/latest/clap/), then find a concrete way of achieving your goal and implement it into ```src/cli.rs``` **following existing syntax**.

Since these options cannot be formally tested in Rust, **always test edge cases** (if present) and document them inside your **PR description**



## Adding config related features
These will use the [serde crate](https://docs.rs/serde/latest/serde/) for parsing TOML files and go into ```src/config.rs```.

To add or modify a module in the ```config.toml``` file:
- **Add a value** inside the struct ```DisplayConfig```
- **Set a default value** inside the ```default()``` function
- **Set the value to true** inside the ```set_all()``` function

To **test a newly added config module** run the following commands:
```
cargo build --release
./target/release/rustfetch --all
```



## Tests
**Always create and run tests** for your implemented features inside ```tests/FILENAME_tests.rs```, tests must include **edge cases** and explicit **error handling**.
