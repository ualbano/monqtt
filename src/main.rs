use std::fs;
use std::time::Duration;
use std::time::SystemTime;
use std::{thread, time};

const SLEEP_TIME: Duration = time::Duration::from_millis(10000);
const LOADAVG_FILE: &str = "/proc/loadavg";

#[derive(Debug)]
struct Value {
    value: f64,
    label: String,
    time: SystemTime,
}

impl Value {
    fn generate(value: f64, label: String) -> Self {
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
            epoch_time, self.label, self.value
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
        load_avg.next().expect("").parse().expect(""),
        String::from("loadavg_1"),
    ));
    values.push(Value::generate(
        load_avg.next().expect("").parse().expect(""),
        String::from("loadavg_5"),
    ));
    values.push(Value::generate(
        load_avg.next().expect("").parse().expect(""),
        String::from("loadavg_15"),
    ));
}

fn generate_system_values() -> Vec<Value> {
    let mut values = Vec::<Value>::new();
    generate_load_avg_values(&mut values);
    values
}

fn print_system_values(system_values: &Vec<Value>) {
    for system_value in system_values {
        println!("{}", system_value.to_string())
    }
}
