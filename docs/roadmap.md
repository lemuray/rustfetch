# Roadmap for rustfetch

This project has gone through countless iterations and it is always changing, favoring better architectures and more idiomatic practices. Here are some goals we intend to reach alongside their immediate necessity:

### Necessity scale
- <code style="color : red">Critical</code> -> Issue needs to be dealt with immediately and released as a patch as soon as the goal is reached. This is usually attributed to security risks and widespread crashes.
- <code style="color : orange">Important</code> -> A primary concern that can worsen user experience in specific cases but does not affect the majority of use cases.
- <code style="color : green">Minor</code> -> Usually a feature that polishes the final product and is safe to handle with no time pressure.

## Version 0.2.0
- ~~Pristine error handling and testing ( <code style="color : orange">Important</code> )~~
- Test Curl script compatibility across systems ( <code style="color : orange">Important</code> )
- ~~get_directory_usage inside shared.rs displays huge numbers on macOS ( <code style="color : orange">Important</code> )~~
- ~~Move CPU usage and name to single line and print according to the TOML file ( <code style="color : green">Minor</code> )~~
- ~~Add GPU name~~ ( <code style="color : green">Minor</code> )  
- ~~Functions inside shared.rs share a System variable, but the variable is refreshed always regardless of what is on or not. This could be fixed by just passing a blank System variable and refreshing what's needed inside the function, though this could worsen performance if a lot of values want to check the same refreshed variable ( <code style="color : green">Minor</code> )~~
- ~~Add CLI command to increase padding on the logo ( <code style="color : green">Minor</code> )~~

## Version 0.3.0
- Add JSON output using a command such as --json ( <code style="color : green">Minor</code> )
- Enable modifying the TOML config file by using CLI flags: ( <code style="color : green">Minor</code> )
    - --toggle = os
    - --reset-config
- Separate logo handling functions in a folder, add bigger and less minimalistic logos, add more distros and transform the ASCII file path to a more idiomatic Path type instead of &str ( <code style="color : green">Minor</code> )
- Add fallback logo in case the logo is not available. Such as linux.txt or check secondary ID for derivate distros (Artix, Kubuntu exc).
- Add support for logos of different color schemes such as Endeavour OS and Gentoo
