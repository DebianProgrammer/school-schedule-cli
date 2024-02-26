pub mod print_data {
    use chrono::NaiveTime;
    use chrono_humanize::HumanTime;
    const ANSI_RESET: &str = "\x1b[0m";
    const ANSI_CYAN: &str = "\x1b[36m";
    const ANSI_GREEN: &str = "\x1b[32m";

    pub fn print_data(day_config: &Vec<(NaiveTime, NaiveTime)>, current_time: NaiveTime, mode: Option<&str>, use_24_hour_format: bool) {
        match mode {
            Some("raw") => print_raw(day_config, current_time, use_24_hour_format),
            Some("relative") => print_relative(day_config, current_time),
            Some("normal") => print_normal(day_config, current_time, use_24_hour_format),
            None => print_normal(day_config, current_time, use_24_hour_format),
            _ => println!("Invalid mode. Please use --help to see valid modes.")
        }
    }

    fn print_normal(day_config: &Vec<(NaiveTime, NaiveTime)>, current_time: NaiveTime, use_24_hour_format: bool) {
        if day_config == &vec![(NaiveTime::from_hms_opt(0, 0, 0).unwrap(), NaiveTime::from_hms_opt(0, 0, 0).unwrap())] {
            println!("No schedule for today");
            return;
        }
        let format = match use_24_hour_format {
            true => "%H:%M",
            _ => "%I:%M%P"
        };
        for (index, timeset) in day_config.iter().enumerate() {
            if compare_times(timeset.0, timeset.1, current_time) {
                let line = format!("{ANSI_CYAN}{item}{ANSI_RESET} -- {ANSI_GREEN}{start} - {end}{ANSI_RESET} -- ONGOING", item=index+1, start=timeset.0.format(format), end=timeset.1.format(format));
                println!("{line}");
                continue;
            }
            let line = format!("{ANSI_CYAN}{item}{ANSI_RESET} -- {ANSI_GREEN}{start} - {end}{ANSI_RESET}", item=index+1, start=timeset.0.format(format), end=timeset.1.format(format));
            println!("{line}");
        }
    }

    fn print_raw(day_config: &Vec<(NaiveTime, NaiveTime)>, current_time: NaiveTime, use_24_hour_format: bool) {
        if day_config == &vec![(NaiveTime::from_hms_opt(0, 0, 0).unwrap(), NaiveTime::from_hms_opt(0, 0, 0).unwrap())] {
            return;
        }
        let format = match use_24_hour_format {
            true => "%H:%M",
            _ => "%I:%M%P"
        };
        for (index, timeset) in day_config.iter().enumerate() {
            if compare_times(timeset.0, timeset.1, current_time) {
                let line = format!("{item} {start} {end} ONGOING", item=index+1, start=timeset.0.format(format), end=timeset.1.format(format));
                println!("{line}");
                continue;
            }
            let line = format!("{item} {start} {end}", item=index+1, start=timeset.0.format(format), end=timeset.1.format(format));
            println!("{line}");
        }
    }

    fn print_relative(day_config: &Vec<(NaiveTime, NaiveTime)>, current_time: NaiveTime) {
        if day_config == &vec![(NaiveTime::from_hms_opt(0, 0, 0).unwrap(), NaiveTime::from_hms_opt(0, 0, 0).unwrap())] {
            println!("No schedule for today");
            return;
        }
        for (index, timeset) in day_config.iter().enumerate() {
            if compare_times(timeset.0, timeset.1, current_time) {
                let line = format!("{ANSI_CYAN}{item}{ANSI_RESET} -- {ANSI_GREEN}{start} - {end}{ANSI_RESET} -- ONGOING", item=index+1, start=HumanTime::from(timeset.0.signed_duration_since(current_time)), end=HumanTime::from(timeset.1.signed_duration_since(current_time)));
                println!("{line}");
                continue;
            }
            let line = format!("{ANSI_CYAN}{item}{ANSI_RESET} -- {ANSI_GREEN}{start} - {end}{ANSI_RESET}", item=index+1, start=HumanTime::from(timeset.0.signed_duration_since(current_time)), end=HumanTime::from(timeset.1.signed_duration_since(current_time)));
            println!("{line}");
        }
    }

    fn compare_times(st: NaiveTime, et: NaiveTime, ct: NaiveTime) -> bool {
        if ct > st && ct < et {
            true
        } else {
            false
        }
    }
}
