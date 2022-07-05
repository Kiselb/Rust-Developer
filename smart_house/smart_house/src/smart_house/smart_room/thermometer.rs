use std::fmt::Write;

use crate::smart_house::smart_room::smart_device::DEVICE_IDENTITY_MIN_LENGTH;
use crate::smart_house::smart_room::SmartDevice;
use crate::smart_house::SmartHouseErrors;

use sdcp::NetConfig;

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
    fn getp(&self) -> sdcp::results::NetResult {
        Ok(vec![])
    }
    fn setp(&self) -> sdcp::results::NetResult {
        Ok(vec![])
    }
}

impl Thermometer {
    pub fn new(name: String, _net_config: NetConfig) -> Result<Self, SmartHouseErrors> {
        if name.len() < DEVICE_IDENTITY_MIN_LENGTH {
            return Err(SmartHouseErrors::InvalidDeviceIdentity);
        }
        Ok(Self {
            name,
            temperature: 0,
        })
    }
    pub fn temperature(&self) -> i8 {
        self.temperature
    }
    pub fn set_temperature(&mut self, temperature: i8) {
        self.temperature = temperature;
    }
}
