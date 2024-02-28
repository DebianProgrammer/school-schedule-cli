use pico_args::Arguments;
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
    let cli_args = match parse_args(pico_args::Arguments::from_env()) {
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

fn parse_args(mut pargs: Arguments) -> Result<Cli, pico_args::Error> {
    //let mut pargs = pico_args::Arguments::from_env();

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

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use super::*;

    #[test]
    fn test_parse_args() {
        let args: Vec<OsString> = vec![OsString::from("school-schedule-cli"), OsString::from("--mode"), OsString::from("raw"), OsString::from("--day"), OsString::from("monday"), OsString::from("--24-hour")];
        let cli_args = parse_args(pico_args::Arguments::from_vec(args)).expect("Could not parse args");
        assert_eq!(cli_args.mode, Some("raw".to_string()));
        assert_eq!(cli_args.day, Some("monday".to_string()));
        assert_eq!(cli_args.use_24_hour_format, true);
    }

    #[test]
    fn test_parse_args_no_args() {
        let args: Vec<OsString> = vec![OsString::from("school-schedule-cli")];
        let cli_args = parse_args(pico_args::Arguments::from_vec(args)).expect("Could not parse args");
        assert_eq!(cli_args.mode, None);
        assert_eq!(cli_args.day, None);
        assert_eq!(cli_args.use_24_hour_format, false);
    }

    #[test]
    fn test_config() {
        let config = parse_config();
        let days = vec!["monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday"];
        for day in days {
            config.get_times(day);
        }
    }

    #[test]
    fn test_print_data() {
        let current_time = chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        let day_config = vec![(chrono::NaiveTime::from_hms_opt(9, 0, 0).expect("Can't parse time"), chrono::NaiveTime::from_hms_opt(10, 0, 0).expect("Can't parse time")), (chrono::NaiveTime::from_hms_opt(17, 0, 0).expect("Can't parse time"), chrono::NaiveTime::from_hms_opt(18, 0, 0).expect("Can't parse time"))];
        print_data(&day_config, current_time, Some("normal"), false, true);
        print_data(&day_config, current_time, Some("raw"), false, true);
        print_data(&day_config, current_time, Some("relative"), false, true);
    }
}
