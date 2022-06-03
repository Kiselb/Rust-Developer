use crate::smart_house::smart_room::SmartDevice;

pub struct ElectricSocket {
    name: String,
    power_consumption: u32,
    status: bool,
}
impl SmartDevice for ElectricSocket {
    fn identity(&self) -> &String {
        &self.name
    }
    fn info(&self) -> String {
        let mut info = String::from("Electric socket: ");
        info.push_str(&self.name);
        if self.status {
            info.push_str(" State: ON");
            info.push_str(" Consumption power: ");
            info.push_str(&self.power_consumption.to_string());
        } else {
            info.push_str(" State: OFF");
        }
        info
    }
}
impl ElectricSocket {
    pub fn new(name: String) -> Self {
        Self {
            name,
            power_consumption: 0,
            status: false,
        }
    }
    pub fn on(&mut self) {
        self.status = true;
    }
    pub fn off(&mut self) {
        self.status = false;
        self.power_consumption = 0;
    }
    pub fn power_consumption(&self) -> u32 {
        self.power_consumption
    }
    pub fn status(&self) -> bool {
        self.status
    }
}
