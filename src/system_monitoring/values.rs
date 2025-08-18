use std::{fmt::Display, time::SystemTime};

#[derive(Debug)]
enum ValueType<T: Display> {
    PERCENT(T),
    SIMPLE(T),
}

impl<T: Display> ValueType<T> {
    fn unit(&self) -> &str {
        match self {
            ValueType::PERCENT(_) => "%",
            _other => "",
        }
    }
}

impl<T: Display> ValueType<T> {
    fn to_str(&self) -> String {
        match self {
            ValueType::PERCENT(value) => format!("{0:.2} {1}", value, self.unit()),
            ValueType::SIMPLE(value) => format!("{0:.2} {1}", value, self.unit()),
        }
    }
}

#[derive(Debug)]
pub struct Value<T: Display> {
    value: ValueType<T>,
    label: String,
    time: SystemTime,
}

impl<T: Display> Value<T> {
    pub fn simple(value: T, label: String) -> Self {
        Value {
            value: ValueType::SIMPLE(value),
            label,
            time: SystemTime::now(),
        }
    }
}

impl<T: Display> Value<T> {
    pub fn percentage(value: T, label: String) -> Self {
        Value {
            value: ValueType::PERCENT(value),
            label,
            time: SystemTime::now(),
        }
    }
}

impl<T: Display> Value<T> {
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
            self.value.to_str()
        ))
    }
}
