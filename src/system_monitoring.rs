pub mod values;

use crate::system_monitoring::values::{Export, PercentageValue};
use std::fs;
use std::time::SystemTime;
use sysinfo::Disks;
use values::SimpleValue;

const LOADAVG_FILE: &str = "/proc/loadavg";

fn generate_load_avg_values(values: &mut Vec<Box<dyn Export>>) {
    let binding = fs::read_to_string(LOADAVG_FILE).expect("Cannot read loadavg file");
    let mut load_avg = binding.split_whitespace();

    values.push(Box::new(SimpleValue {
        value: load_avg
            .next()
            .expect("Cannot read bind")
            .trim()
            .parse()
            .expect("Cannot parse element"),
        label: String::from("loadavg/1"),
        time: SystemTime::now(),
    }));
    values.push(Box::new(SimpleValue {
        value: load_avg
            .next()
            .expect("Cannot read bind")
            .trim()
            .parse()
            .expect("Cannot parse element"),
        label: String::from("loadavg/5"),
        time: SystemTime::now(),
    }));
    values.push(Box::new(SimpleValue {
        value: load_avg
            .next()
            .expect("Cannot read bind")
            .trim()
            .parse()
            .expect("Cannot parse element"),
        label: String::from("loadavg/15"),
        time: SystemTime::now(),
    }));
}

fn generate_disk_informations(values: &mut Vec<Box<dyn Export>>) {
    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        let available_space = disk.available_space() as f32 / disk.total_space() as f32;
        let disk_name: String = String::from(
            disk.name()
                .to_str()
                .expect("Cannot convert disk name to string"),
        );

        let disk_name: String = disk_name
            .split("/")
            .last()
            .expect("Cannot parse disk name")
            .parse()
            .expect("Cannot parse disk name");
        values.push(Box::new(PercentageValue {
            value: available_space,
            label: String::from(format!("available_space/{}", disk_name)),
            time: SystemTime::now(),
        }));
    }
}

pub fn generate_system_values() -> Vec<Box<dyn Export>> {
    let mut values = Vec::<Box<dyn Export>>::new();
    generate_load_avg_values(&mut values);
    generate_disk_informations(&mut values);
    values
}

pub fn print_system_values(system_values: &Vec<Box<dyn Export>>) {
    for system_value in system_values {
        println!("{}", system_value.export_to_string())
    }
}
