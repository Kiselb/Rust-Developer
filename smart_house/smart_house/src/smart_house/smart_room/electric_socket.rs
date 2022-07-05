use std::fmt::Write;

use crate::smart_house::smart_room::smart_device::DEVICE_IDENTITY_MIN_LENGTH;
use crate::smart_house::smart_room::SmartDevice;
use crate::smart_house::SmartHouseErrors;

use sdcp::NetConfig;

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
    fn getp(&self) -> sdcp::results::NetResult {
        Ok(vec![])
    }
    fn setp(&self) -> sdcp::results::NetResult {
        Ok(vec![])
    }
}

impl ElectricSocket {
    pub fn new(name: String, _net_config: NetConfig) -> Result<Self, SmartHouseErrors> {
        if name.len() < DEVICE_IDENTITY_MIN_LENGTH {
            return Err(SmartHouseErrors::InvalidDeviceIdentity);
        }
        Ok(Self {
            name,
            power_consumption: 0,
            status: false,
        })
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
