use std::fs;
use std::time::Duration;
use std::time::SystemTime;
use std::{thread, time};
use sysinfo::Disks;

const SLEEP_TIME: Duration = time::Duration::from_millis(10000);
const LOADAVG_FILE: &str = "/proc/loadavg";

#[derive(Debug)]
enum ValueType {
    PERCENT(f32),
    SIMPLE(f32),
}

impl ValueType {
    fn unit(&self) -> &str {
        match self {
            ValueType::PERCENT(_) => "%",
            _other => "",
        }
    }
}

impl ValueType {
    fn to_str(&self) -> String {
        match self {
            ValueType::PERCENT(value) => format!("{0:.2} {1}", value, self.unit()),
            ValueType::SIMPLE(value) => format!("{0:.2} {1}", value, self.unit()),
        }
    }
}

#[derive(Debug)]
struct Value {
    value: ValueType,
    label: String,
    time: SystemTime,
}

impl Value {
    fn generate(value: ValueType, label: String) -> Self {
        Value {
            value,
            label,
            time: SystemTime::now(),
        }
    }
}

impl Value {
    fn to_string(&self) -> String {
        let epoch_time = self
            .time
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Failed to parse time")
            .as_secs();
        String::from(format!(
            "[{0}] {1}: {2}",
            epoch_time,
            self.label,
            self.value.to_str()
        ))
    }
}

fn main() {
    loop {
        let system_values = generate_system_values();
        print_system_values(&system_values);
        thread::sleep(SLEEP_TIME);
    }
}

fn generate_load_avg_values(values: &mut Vec<Value>) {
    let binding = fs::read_to_string(LOADAVG_FILE).expect("Cannot read loadavg file");
    let mut load_avg = binding.split_whitespace();

    values.push(Value::generate(
        ValueType::SIMPLE(load_avg.next().expect("").parse().expect("")),
        String::from("loadavg/1"),
    ));
    values.push(Value::generate(
        ValueType::SIMPLE(load_avg.next().expect("").parse().expect("")),
        String::from("loadavg/5"),
    ));
    values.push(Value::generate(
        ValueType::SIMPLE(load_avg.next().expect("").parse().expect("")),
        String::from("loadavg/"),
    ));
}

fn generate_disk_informations(values: &mut Vec<Value>) {
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
        values.push(Value::generate(
            ValueType::PERCENT(available_space),
            String::from(format!("available_space/{}", disk_name)),
        ));
    }
}
fn generate_system_values() -> Vec<Value> {
    let mut values = Vec::<Value>::new();
    generate_load_avg_values(&mut values);
    generate_disk_informations(&mut values);
    values
}

fn print_system_values(system_values: &Vec<Value>) {
    for system_value in system_values {
        println!("{}", system_value.to_string())
    }
}
