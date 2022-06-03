use std::collections::HashMap;
pub mod smart_device;
pub mod electric_socket;
pub mod thermometer;

use crate::smart_house::smart_room::smart_device::SmartDevice;

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
    pub fn info(&self) {
        println!("Room: {}", &self.name);
        for val in &mut self.devices.values() {
            println!("{}", val.info());
        }
    }
    pub fn add(&mut self, device: Box<dyn SmartDevice>) {
        self.devices.insert(String::from(device.identity()), device);
    }
    pub fn get(&self, device_name: &str) -> Option<&dyn SmartDevice> {
        self.devices.get(device_name).map(|v| v.as_ref())
    }
}
