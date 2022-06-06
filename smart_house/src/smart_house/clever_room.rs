pub mod clever_device;

use std::collections::HashMap;
use std::fmt::Write;

use crate::smart_house::clever_room::clever_device::CleverDevice;

use super::smart_room::smart_device::SmartDevice;

pub struct CleverRoom {
    pub name: String,
    pub devices: HashMap<String, CleverDevice>,
}
impl CleverRoom {
    pub fn new(name: String) -> Self {
        Self {
            name,
            devices: HashMap::new(),
        }
    }
    pub fn info(&self) -> String {
        let mut devices: Vec<_> = self.devices.iter().collect();
        devices.sort_by(|op1, op2| op1.0.cmp(op2.0));

        let mut info = String::from("");
        for device in devices {
            match device.1 {
                CleverDevice::ElecticSocket(electric_socket) => {
                    writeln!(info, "{}", &electric_socket.info()).unwrap()
                }
                CleverDevice::Thermometer(thermometer) => {
                    writeln!(info, "{}", &thermometer.info()).unwrap()
                }
            }
        }
        info
    }
    pub fn add(&mut self, device: CleverDevice) {
        match device {
            CleverDevice::ElecticSocket(electric_socket) => {
                self.devices.insert(
                    String::from(electric_socket.identity()),
                    CleverDevice::ElecticSocket(electric_socket),
                );
            }
            CleverDevice::Thermometer(thermometer) => {
                self.devices.insert(
                    String::from(thermometer.identity()),
                    CleverDevice::Thermometer(thermometer),
                );
            }
        }
    }
    pub fn get(&self, device_name: &str) -> Option<&CleverDevice> {
        self.devices.get(device_name)
    }
}
