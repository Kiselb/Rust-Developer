use std::collections::HashMap;
use std::fmt::Write;

pub mod electric_socket;
pub mod smart_device;
pub mod thermometer;

use crate::smart_house::errors::SmartHouseErrors;
use crate::smart_house::smart_room::smart_device::{SmartDevice, DEVICE_IDENTITY_MIN_LENGTH};

pub const SMART_ROOM_NAME_MIN_LENGTH: usize = 8;

pub struct SmartRoom {
    pub name: String,
    pub devices: HashMap<String, Box<dyn SmartDevice>>,
}
impl SmartRoom {
    pub fn new(name: String) -> Result<Self, SmartHouseErrors> {
        if name.len() < SMART_ROOM_NAME_MIN_LENGTH {
            return Err(SmartHouseErrors::InvalidRoomName);
        }
        Ok(Self {
            name,
            devices: HashMap::new(),
        })
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
    pub fn add(&mut self, device: Box<dyn SmartDevice>) -> Result<(), SmartHouseErrors> {
        let device_name = String::from(device.identity());
        if device_name.len() < DEVICE_IDENTITY_MIN_LENGTH {
            return Err(SmartHouseErrors::InvalidDeviceIdentity);
        }
        self.devices.insert(device_name, device);
        Ok(())
    }
    pub fn rem(&mut self, device_name: &str) -> Option<Box<dyn SmartDevice>> {
        self.devices.remove_entry(device_name).map(|(_, room)| room)
    }
    pub fn get(&self, device_name: &str) -> Option<&dyn SmartDevice> {
        self.devices.get(device_name).map(|v| v.as_ref())
    }
    pub fn list(&self) -> Vec<String> {
        self.devices.keys().cloned().collect::<Vec<String>>()
    }
}
