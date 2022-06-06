use crate::smart_house::smart_room::SmartDevice;
use std::fmt::Write;

pub struct Thermometer {
    name: String,
    temperature: i8,
}
impl SmartDevice for Thermometer {
    fn identity(&self) -> &String {
        &self.name
    }
    fn info(&self) -> String {
        let mut info = String::new();
        write!(
            info,
            "Thermometer: {} Value: {}",
            &self.name,
            &self.temperature.to_string()
        )
        .unwrap();
        info
    }
}
impl Thermometer {
    pub fn new(name: String) -> Self {
        Self {
            name,
            temperature: 0,
        }
    }
    pub fn temperature(&self) -> i8 {
        self.temperature
    }
    pub fn set_temperature(&mut self, temperature: i8) {
        self.temperature = temperature;
    }
}
