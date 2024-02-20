# school-schedule-cli
school-schedule-cli is a simple CLI tool to get your school schedule.

## Building and Running
You will need the latest stable rust. The easiest way to install that is using `rustup`
```
git clone https://github.com/DebianProgrammer/school-schedule-cli
cd school-schedule-cli
cargo run --release
```
The binary should be at `target/release/school-schedule-cli`

## Configuration
The `config.toml` in this repo is the default configuration and also acts as an example. To edit your schedule copy the file into `$XDG_CONFIG_HOME/school-schedule-cli/config.toml` and edit it.

### Syntax
Every day of the week is in the configuration file. Periods are defined by time periods in 24-hour time (e.g. `08:00-09:00`) and separated by commas. You can add as much periods as you want. If you wish to have no schedule for a certain day, do not delete the day, instead set its `times` to a empty string.

## Usage
Simply run the binary and it will print the schedule for the current day. The program has 2 modes. If you wish to parse your current day schedule to use with other programs, you should use the `raw` mode for easier parsing. It can be used by passing `--mode raw` to the binary.

