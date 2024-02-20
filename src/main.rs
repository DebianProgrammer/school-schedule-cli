use crate::config::*;
use crate::print_data::print_data::print_data;
mod print_data;
mod config;

#[derive(Debug)]
struct Cli {
    mode: Option<String>
}

fn main() {
    let cli_args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    let current_day = chrono::offset::Local::now().format("%A").to_string().to_lowercase();
    let current_time = chrono::offset::Local::now().naive_local().time();
    let config = parse_config();
    let current_day_config = config.get_times(&current_day);

    print_data(current_day_config, current_time, cli_args.mode.as_deref());
}

fn parse_args() -> Result<Cli, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();
    let args = Cli {
        mode: pargs.opt_value_from_str("--mode")?
    };

    Ok(args)
}
