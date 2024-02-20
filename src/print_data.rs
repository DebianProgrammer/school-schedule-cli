pub mod print_data {
    use chrono::NaiveTime;
    const ANSI_RESET: &str = "\x1b[0m";
    const ANSI_CYAN: &str = "\x1b[36m";
    const ANSI_GREEN: &str = "\x1b[32m";

    pub fn print_data(day_config: &Vec<(NaiveTime, NaiveTime)>, current_time: NaiveTime, mode: Option<&str>) {
        match mode {
            Some("raw") => print_raw(day_config, current_time),
            _ => print_normal(day_config, current_time)
        }
    }

    fn print_normal(day_config: &Vec<(NaiveTime, NaiveTime)>, current_time: NaiveTime) {
        if day_config == &vec![(NaiveTime::from_hms_opt(0, 0, 0).unwrap(), NaiveTime::from_hms_opt(0, 0, 0).unwrap())] {
            println!("No schedule for today");
            return;
        }
        for (index, timeset) in day_config.iter().enumerate() {
            if compare_times(timeset.0, timeset.1, current_time) {
                let line = format!("{ANSI_CYAN}{item}{ANSI_RESET} -- {ANSI_GREEN}{start} - {end}{ANSI_RESET} -- ONGOING", item=index+1, start=timeset.0, end=timeset.1);
                println!("{line}");
                continue;
            }
            let line = format!("{ANSI_CYAN}{item}{ANSI_RESET} -- {ANSI_GREEN}{start} - {end}{ANSI_RESET}", item=index+1, start=timeset.0, end=timeset.1);
            println!("{line}");
        }
    }

    fn print_raw(day_config: &Vec<(NaiveTime, NaiveTime)>, current_time: NaiveTime) {
        if day_config == &vec![(NaiveTime::from_hms_opt(0, 0, 0).unwrap(), NaiveTime::from_hms_opt(0, 0, 0).unwrap())] {
            return;
        }
        for (index, timeset) in day_config.iter().enumerate() {
            if compare_times(timeset.0, timeset.1, current_time) {
                let line = format!("{item} {start} {end} ONGOING", item=index+1, start=timeset.0, end=timeset.1);
                println!("{line}");
                continue;
            }
            let line = format!("{item} {start} {end}", item=index+1, start=timeset.0, end=timeset.1);
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
