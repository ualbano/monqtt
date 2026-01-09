use std::time::SystemTime;

fn format_string<T>(time: &SystemTime, label: &String, value: T, unit: char) -> String
where
    T: std::fmt::Display,
{
    let epoch_time = time
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to parse time")
        .as_secs();

    String::from(format!(
        "[{0}] {1}: {2:.2} {3}",
        epoch_time, label, value, unit
    ))
}

pub trait Export {
    fn export_to_string(&self) -> String;
}

#[derive(Debug)]
pub struct SimpleValue {
    pub value: f32,
    pub label: String,
    pub time: SystemTime,
}

impl Export for SimpleValue {
    fn export_to_string(&self) -> String {
        format_string(&self.time, &self.label, &self.value, ' ')
    }
}

#[derive(Debug)]
pub struct PercentageValue {
    pub value: f32,
    pub label: String,
    pub time: SystemTime,
}

impl Export for PercentageValue {
    fn export_to_string(&self) -> String {
        format_string(&self.time, &self.label, &self.value, '%')
    }
}
