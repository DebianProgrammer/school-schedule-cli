mod structs;
use structs::Config;
use std::fs::{self};
use chrono::{self, NaiveTime};

#[derive(Debug)]
pub struct Schedule {
    monday: Vec<(NaiveTime, NaiveTime)>,
    tuesday: Vec<(NaiveTime, NaiveTime)>,
    wednesday: Vec<(NaiveTime, NaiveTime)>,
    thursday: Vec<(NaiveTime, NaiveTime)>,
    friday: Vec<(NaiveTime, NaiveTime)>,
    saturday: Vec<(NaiveTime, NaiveTime)>,
    sunday: Vec<(NaiveTime, NaiveTime)>
}

impl Schedule {
    pub fn get_times(&self, day: &str) -> &Vec<(NaiveTime, NaiveTime)> {
        match day {
            "monday" => &self.monday,
            "tuesday" => &self.tuesday,
            "wednesday" => &self.wednesday,
            "thursday" => &self.thursday,
            "friday" => &self.friday,
            "saturday" => &self.saturday,
            "sunday" => &self.sunday,
            _ => {
                eprintln!("Invalid day. Valid options are: sunday, monday, tuesday, wednesday, thursday, friday, saturday");
                std::process::exit(1);
            }
        }
    }
}

pub fn parse_config() -> Schedule {
    let file = xdg::BaseDirectories::with_prefix("school-schedule-cli").expect("Could not find base directories")
        .find_config_file("config.toml")
        .and_then(|path| fs::read_to_string(path).ok())
        .unwrap_or_else(|| include_str!("../../config.toml").to_string());

    let config_toml: Config = match toml::from_str(&file) {
        Ok(config) => config,
        Err(e) => panic!("Could not parse config.toml: {}", e)
    };

    Schedule {
        monday: config_toml.monday.times.split(",").map(|time| {
            if time == "" {
                return (NaiveTime::from_hms_opt(0, 0, 0).unwrap(), NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            }
            let times: Vec<&str> = time.split("-").collect();
            if times.len() != 2 {
                panic!("Invalid time format")
            }
            (NaiveTime::parse_from_str(times[0], "%H:%M").expect("Could not parse time"), NaiveTime::parse_from_str(times[1], "%H:%M").expect("Could not parse time"))
        }).collect(),

        tuesday: config_toml.tuesday.times.split(",").map(|time| {
            if time == "" {
                return (NaiveTime::from_hms_opt(0, 0, 0).unwrap(), NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            }
            let times: Vec<&str> = time.split("-").collect();
            if times.len() != 2 {
                panic!("Invalid time format")
            }
            (NaiveTime::parse_from_str(times[0], "%H:%M").expect("Could not parse time"), NaiveTime::parse_from_str(times[1], "%H:%M").expect("Could not parse time"))
        }).collect(),

        wednesday: config_toml.wednesday.times.split(",").map(|time| {
            if time == "" {
                return (NaiveTime::from_hms_opt(0, 0, 0).unwrap(), NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            }
            let times: Vec<&str> = time.split("-").collect();
            if times.len() != 2 {
                panic!("Invalid time format")
            }
            (NaiveTime::parse_from_str(times[0], "%H:%M").expect("Could not parse time"), NaiveTime::parse_from_str(times[1], "%H:%M").expect("Could not parse time"))
        }).collect(),

        thursday: config_toml.thursday.times.split(",").map(|time| {
            if time == "" {
                return (NaiveTime::from_hms_opt(0, 0, 0).unwrap(), NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            }
            let times: Vec<&str> = time.split("-").collect();
            if times.len() != 2 {
                panic!("Invalid time format")
            }
            (NaiveTime::parse_from_str(times[0], "%H:%M").expect("Could not parse time"), NaiveTime::parse_from_str(times[1], "%H:%M").expect("Could not parse time"))
        }).collect(),

        friday: config_toml.friday.times.split(",").map(|time| {
            if time == "" {
                return (NaiveTime::from_hms_opt(0, 0, 0).unwrap(), NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            }
            let times: Vec<&str> = time.split("-").collect();
            if times.len() != 2 {
                panic!("Invalid time format")
            }
            (NaiveTime::parse_from_str(times[0], "%H:%M").expect("Could not parse time"), NaiveTime::parse_from_str(times[1], "%H:%M").expect("Could not parse time"))
        }).collect(),

        saturday: config_toml.saturday.times.split(",").map(|time| {
            if time == "" {
                return (NaiveTime::from_hms_opt(0, 0, 0).unwrap(), NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            }
            let times: Vec<&str> = time.split("-").collect();
            if times.len() != 2 {
                panic!("Invalid time format")
            }
            (NaiveTime::parse_from_str(times[0], "%H:%M").expect("Could not parse time"), NaiveTime::parse_from_str(times[1], "%H:%M").expect("Could not parse time"))
        }).collect(),

        sunday: config_toml.sunday.times.split(",").map(|time| {
            if time == "" {
                return (NaiveTime::from_hms_opt(0, 0, 0).unwrap(), NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            }
            let times: Vec<&str> = time.split("-").collect();
            if times.len() != 2 {
                panic!("Invalid time format")
            }
            (NaiveTime::parse_from_str(times[0], "%H:%M").expect("Could not parse time"), NaiveTime::parse_from_str(times[1], "%H:%M").expect("Could not parse time"))
        }).collect(),

    }
}
