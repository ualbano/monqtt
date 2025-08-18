pub mod values;

use std::fs;
use sysinfo::Disks;
use values::Value;

const LOADAVG_FILE: &str = "/proc/loadavg";

fn generate_load_avg_values(values: &mut Vec<Value<f32>>) {
    let binding = fs::read_to_string(LOADAVG_FILE).expect("Cannot read loadavg file");
    let mut load_avg = binding.split_whitespace();

    values.push(Value::simple(
        load_avg.next().expect("").parse().expect(""),
        String::from("loadavg/1"),
    ));
    values.push(Value::simple(
        load_avg.next().expect("").parse().expect(""),
        String::from("loadavg/5"),
    ));
    values.push(Value::simple(
        load_avg.next().expect("").parse().expect(""),
        String::from("loadavg/"),
    ));
}

fn generate_disk_informations(values: &mut Vec<Value<f32>>) {
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
        values.push(Value::percentage(
            available_space,
            String::from(format!("available_space/{}", disk_name)),
        ));
    }
}
pub fn generate_system_values() -> Vec<Value<f32>> {
    let mut values = Vec::<Value<f32>>::new();
    generate_load_avg_values(&mut values);
    generate_disk_informations(&mut values);
    values
}

pub fn print_system_values(system_values: &Vec<Value<f32>>) {
    for system_value in system_values {
        println!("{}", system_value.to_string())
    }
}
