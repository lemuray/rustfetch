# Roadmap for rustfetch

This project has gone through countless iterations and it is always changing, favoring better architectures and more idiomatic practices. Here are some goals we intend to reach alongside their immediate necessity:

### Necessity scale
- <code style="color : red">Critical</code> -> Issue needs to be dealt with immediately and released as a patch as soon as the goal is reached. This is usually attributed to security risks and widespread crashes.
- <code style="color : orange">Important</code> -> A primary concern that can worsen user experience in specific cases but does not affect the majority of use cases.
- <code style="color : green">Minor</code> -> Usually a feature that polishes the final product and is safe to handle with no time pressure.

## Version 1.0.0
- First official stable version, therefore it must handle every error correctly and predictably ( <code style="color:red">Critical</code> )
- Support for bigger logos (i.e: enough information to fit a bigger logo without it seeming overkill) ( <code style="color : orange">Important</code> )
- Bulletproof installation script, add logs while installing as well for a more informative installation ( <code style="color : orange">Important</code> )
- ~~Runtime comparable to fastfetch's ( <code style="color : orange">Important</code> )~~
- Support for Redox OS ( <code style="color : green">Minor</code> )
- ~~Concrete speed comparisons (and tests) with neofetch and fastfetch to place in the main README ( <code style="color : green">Minor</code> )~~
- Add JSON output using a command such as --json ( <code style="color : green">Minor</code> )
- Enable modifying the TOML config file by using CLI flags: ( <code style="color : green">Minor</code> )
    - --toggle = os
    - ~~--reset-config~~

## Version 0.4.0
- Add fallback logo in case the logo is not available. Such as linux.txt or check secondary ID for derivate distros (Artix, Kubuntu exc) ( <code style="color : orange">Important</code> )
- Add support for logos of different color schemes such as Endeavour OS and Gentoo ( <code style="color : orange">Important</code> )
- Separate logo handling functions in a folder, add more distros and transform the ASCII file path to a more idiomatic Path type instead of &str ( <code style="color : green">Minor</code> )

## Version 0.3.0 - RELEASED (19 Feb, 2026)
- ~~Add caching system in order to substantially decrease runtime ( <code style="color : orange">Important</code> )~~
- ~~Add host name and username ( <code style="color : green">Minor</code> )~~
- ~~Thin down dependencies ( <code style="color : green">Minor</code> )~~
- ~~Change Cargo release profile in order to get faster runtimes ( <code style="color : green">Minor</code> )~~
- ~~Add screen resolution and refresh rate ( <code style="color : green">Minor</code> )~~

## Version 0.2.0 - RELEASED (10 Feb, 2026)
- ~~Pristine error handling and testing ( <code style="color : orange">Important</code> )~~
- ~~Test Curl script compatibility across systems ( <code style="color : orange">Important</code> )~~
- ~~get_directory_usage inside shared.rs displays huge numbers on macOS ( <code style="color : orange">Important</code> )~~
- ~~Move CPU usage and name to single line and print according to the TOML file ( <code style="color : green">Minor</code> )~~
- ~~Add GPU name ( <code style="color : green">Minor</code> )~~
- ~~Functions inside shared.rs share a System variable, but the variable is refreshed always regardless of what is on or not. This could be fixed by just passing a blank System variable and refreshing what's needed inside the function, though this could worsen performance if a lot of values want to check the same refreshed variable ( <code style="color : green">Minor</code> )~~
- ~~Add CLI command to increase padding on the logo ( <code style="color : green">Minor</code> )~~
