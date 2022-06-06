use std::fmt::Write;

pub mod electric_socket;
pub mod smart_device;
pub mod thermometer;

use crate::smart_house::smart_room::smart_device::SmartDevice;
use std::collections::HashMap;

pub struct SmartRoom {
    pub name: String,
    pub devices: HashMap<String, Box<dyn SmartDevice>>,
}
impl SmartRoom {
    pub fn new(name: String) -> Self {
        Self {
            name,
            devices: HashMap::new(),
        }
    }
    pub fn info(&self) -> String {
        let mut devices: Vec<_> = self.devices.iter().collect();
        devices.sort_by(|op1, op2| op1.0.cmp(op2.0));

        let mut info = String::new();
        for val in devices {
            writeln!(info, "{}", &val.1.info()).unwrap();
        }
        info
    }
    pub fn add(&mut self, device: Box<dyn SmartDevice>) {
        self.devices.insert(String::from(device.identity()), device);
    }
    pub fn get(&self, device_name: &str) -> Option<&dyn SmartDevice> {
        self.devices.get(device_name).map(|v| v.as_ref())
    }
}
