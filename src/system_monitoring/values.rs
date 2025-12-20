use std::time::SystemTime;

use crate::system_monitoring::types::{Export, Percentage, Simple};

#[derive(Debug)]
pub struct Value<T: Export> {
    value: T,
    label: String,
    time: SystemTime,
}

impl Value<Simple> {
    pub fn simple(value: f32, label: String) -> Value<Simple> {
        Value {
            value: Simple { value: value },
            label,
            time: SystemTime::now(),
        }
    }
}

impl Value<Percentage> {
    pub fn percentage(value: f32, label: String) -> Self {
        Value {
            value: Percentage { value: value },
            label,
            time: SystemTime::now(),
        }
    }
}

impl<T: Export> Value<T> {
    pub fn to_string(&self) -> String {
        let epoch_time = self
            .time
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Failed to parse time")
            .as_secs();
        String::from(format!(
            "[{0}] {1}: {2}",
            epoch_time,
            self.label,
            self.value.export_to_string()
        ))
    }
}
