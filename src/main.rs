use crate::config::*;
use crate::print_data::print_data::print_data;
mod print_data;
mod config;

const HELP_STRING: &str = "\
school-schedule-cli is a simple CLI tool to show the current day's schedule. It uses a config file to get the schedule for the current day.

USAGE:
    school-schedule-cli [OPTIONS]

OPTIONS:
    --mode <mode>       The mode to use for the schedule.
    --24-hour           Use 24-hour format for the time. If not provided, 12-hour format will be used.
    --help              Prints help information and exits

MODES:
    normal              Print the schedule in a normal human-readable format. This is the default mode.
    raw                 Print the schedule in a raw format. This is useful for scripting.
    relative            Print the schedule in a relative format. This is useful for seeing how much time is left until the next class.

EXAMPLES:
    school-schedule-cli
    school-schedule-cli --mode raw
    school-schedule-cli --24-hour
    school-schedule-cli --mode raw --24-hour

";

#[derive(Debug)]
struct Cli {
    mode: Option<String>,
    day: Option<String>,
    use_24_hour_format: bool
}

fn main() {
    let cli_args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    let current_day = match &cli_args.day {
        Some(day) => day.to_lowercase(),
        None => chrono::offset::Local::now().format("%A").to_string().to_lowercase()
    };

    let current_time = chrono::offset::Local::now().naive_local().time();
    let config = parse_config();
    let current_day_config = config.get_times(&current_day);

    print_data(current_day_config, current_time, cli_args.mode.as_deref(), cli_args.use_24_hour_format, cli_args.day.is_none());
}

fn parse_args() -> Result<Cli, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP_STRING);
        std::process::exit(0);
    }

    let args = Cli {
        mode: pargs.opt_value_from_str("--mode")?,
        day: pargs.opt_value_from_str("--day")?,
        use_24_hour_format: pargs.contains("--24-hour")
    };

    Ok(args)
}
