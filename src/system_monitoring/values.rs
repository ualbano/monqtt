use std::time::SystemTime;

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
pub struct Value {
    value: ValueType,
    label: String,
    time: SystemTime,
}

impl Value {
    pub fn simple(value: f32, label: String) -> Self {
        Value {
            value: ValueType::SIMPLE(value),
            label,
            time: SystemTime::now(),
        }
    }
}

impl Value {
    pub fn percentage(value: f32, label: String) -> Self {
        Value {
            value: ValueType::PERCENT(value),
            label,
            time: SystemTime::now(),
        }
    }
}

impl Value {
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
