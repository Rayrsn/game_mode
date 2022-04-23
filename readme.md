<div align="center">
  
# `🎮 game_mode`
## **A program that executes a set of commands and scripts whenever a game is launched - Written in pure Rust 🦀**

</div>

## Features 🏆
* Set the laptop fan speed to maximum.
  * Has to be set manually in the config file.
  * `nbfc` should be installed and added to the path.
* Run a custom command.
  * Personally I execute my own [pingr](https://github.com/Rayrsn/pingr) script to check my ping.
* Keep hard drive alive.
  * Does this by writing to a temporary file every 5 seconds. (Doesn't affect disk storage)

## Building 🔨

```bash 
git clone https://github.com/Rayrsn/game_mode
cd game_mode
cargo build --release
```
***Alternatively:***  You can replace `cargo build --release` with `cargo install --path .` to install the binary. (Build and add to path.)

## Usage 📚

* You can run `game_mode -h` to see the possible launch arguments (The default value for every argument is ***true***).
1. Add `game_mode &` to the beginning of Launch Options of your Steam game. (***game_mode*** has to be added to path)
   * (e.g. adding `game_mode -c &` to the beginning of Launch Options will cause the program to not execute a custom command)
1. Enjoy!
