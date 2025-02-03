use std::time::Duration;
use std::{thread, time};

const SLEEP_TIME: Duration = time::Duration::from_millis(10000);

pub mod system_monitoring;

fn main() {
    loop {
        let system_values = system_monitoring::generate_system_values();
        system_monitoring::print_system_values(&system_values);
        thread::sleep(SLEEP_TIME);
    }
}
