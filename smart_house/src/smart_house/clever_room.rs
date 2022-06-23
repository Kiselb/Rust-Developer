pub mod clever_device;

use std::collections::HashMap;
use std::fmt::Write;

use crate::smart_house::clever_room::clever_device::CleverDevice;
use crate::smart_house::errors::SmartHouseErrors;
use crate::smart_house::smart_room::smart_device::SmartDevice;
use crate::smart_house::smart_room::smart_device::DEVICE_IDENTITY_MIN_LENGTH;

pub const CLEVER_ROOM_NAME_MIN_LENGTH: usize = 8;

pub struct CleverRoom {
    pub name: String,
    pub devices: HashMap<String, CleverDevice>,
}

impl CleverRoom {
    pub fn new(name: String) -> Result<Self, SmartHouseErrors> {
        if name.len() < CLEVER_ROOM_NAME_MIN_LENGTH {
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
    pub fn add(&mut self, device: CleverDevice) -> Result<(), SmartHouseErrors> {
        match device {
            CleverDevice::ElecticSocket(electric_socket) => {
                if electric_socket.identity().len() < DEVICE_IDENTITY_MIN_LENGTH {
                    return Err(SmartHouseErrors::InvalidDeviceIdentity);
                }
                self.devices.insert(
                    String::from(electric_socket.identity()),
                    CleverDevice::ElecticSocket(electric_socket),
                );
                Ok(())
            }
            CleverDevice::Thermometer(thermometer) => {
                if thermometer.identity().len() < DEVICE_IDENTITY_MIN_LENGTH {
                    return Err(SmartHouseErrors::InvalidDeviceIdentity);
                }
                self.devices.insert(
                    String::from(thermometer.identity()),
                    CleverDevice::Thermometer(thermometer),
                );
                Ok(())
            }
        }
    }

    pub fn get(&self, device_name: &str) -> Option<&CleverDevice> {
        self.devices.get(device_name)
    }

    pub fn get_mut(&mut self, device_name: &str) -> Option<&mut CleverDevice> {
        self.devices.get_mut(device_name)
    }

    pub fn rem(&mut self, device_name: &str) -> Option<CleverDevice> {
        self.devices.remove_entry(device_name).map(|(_, room)| room)
    }
    
    pub fn list(&self) -> Vec<String> {
        self.devices.keys().cloned().collect::<Vec<String>>()
    }
}
