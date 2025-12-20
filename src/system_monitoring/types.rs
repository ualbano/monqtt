use std::fmt::format;

pub trait Export {
    fn export_to_string(&self) -> String;
}

pub struct Simple {
    pub value: f32,
}

impl Export for Simple {
    fn export_to_string(&self) -> String {
        format!("{0:.2}", self.value)
    }
}

pub struct Percentage {
    pub value: f32,
}

impl Export for Percentage {
    fn export_to_string(&self) -> String {
        format!("{0:.2} %", self.value)
    }
}
