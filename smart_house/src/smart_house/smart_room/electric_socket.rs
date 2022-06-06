use crate::smart_house::smart_room::SmartDevice;
use std::fmt::Write;

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
        let mut info = String::new();
        write!(info, "Electric socket: {}", &self.name).unwrap();
        if self.status {
            write!(
                info,
                " State: ON Consumption power: {}",
                &self.power_consumption
            )
            .unwrap();
        } else {
            write!(info, " State: OFF").unwrap();
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
