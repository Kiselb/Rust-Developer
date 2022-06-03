use crate::smart_house::smart_room::SmartDevice;

pub struct Thermometer {
    name: String,
    temperature: i8,
}
impl SmartDevice for Thermometer {
    fn identity(&self) -> &String {
        &self.name
    }
    fn info(&self) -> String {
        let mut info = String::from("Thermometer: ");
        info.push_str(&self.name);
        info.push_str(" Value: ");
        info.push_str(&self.temperature.to_string());
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
}
