use std::collections::HashMap;
use std::fmt::Write;

use crate::smart_house::clever_room::clever_device::CleverDevice;
use crate::smart_house::clever_room::CleverRoom;
use crate::smart_house::clever_room::CLEVER_ROOM_NAME_MIN_LENGTH;
use crate::smart_house::errors::SmartHouseErrors;
use crate::smart_house::smart_room::smart_device::SmartDevice;

pub const CLEVER_HOUSE_NAME_MIN_LENGTH: usize = 8;

pub struct CleverHouse {
    name: String,
    rooms: HashMap<String, CleverRoom>,
}
impl CleverHouse {
    pub fn new(name: String) -> Result<Self, SmartHouseErrors> {
        if name.len() < CLEVER_HOUSE_NAME_MIN_LENGTH {
            return Err(SmartHouseErrors::InvalidHouseName);
        };
        Ok(Self {
            name,
            rooms: HashMap::new(),
        })
    }
    pub fn add(&mut self, room: CleverRoom) -> Result<(), SmartHouseErrors> {
        let room_name = String::from(&room.name);
        if room_name.len() < CLEVER_ROOM_NAME_MIN_LENGTH {
            return Err(SmartHouseErrors::InvalidRoomName);
        }
        self.rooms.insert(String::from(&room.name), room);
        Ok(())
    }
    pub fn rem(&mut self, room_name: &str) -> Option<CleverRoom> {
        self.rooms.remove_entry(room_name).map(|(_, room)| room)
    }
    pub fn get(&self, room_name: &str) -> Option<&CleverRoom> {
        self.rooms.get(room_name)
    }
    pub fn get_mut(&mut self, room_name: &str) -> Option<&mut CleverRoom> {
        self.rooms.get_mut(room_name)
    }
    pub fn list(&self) -> Vec<String> {
        self.rooms.keys().cloned().collect::<Vec<String>>()
    }
    pub fn info_rooms(&self) -> String {
        let mut info = String::new();
        let mut rooms: Vec<_> = self.rooms.iter().collect();
        rooms.sort_by(|op1, op2| op1.0.cmp(op2.0));
        for val in rooms {
            writeln!(info, "Room: {}", val.0).unwrap();
        }
        info
    }
    pub fn info(&self) -> String {
        let mut info = String::new();
        writeln!(info, "House '{}'", &self.name).unwrap();
        let mut rooms: Vec<_> = self.rooms.iter().collect();
        rooms.sort_by(|op1, op2| op1.0.cmp(op2.0));
        for val in rooms {
            write!(info, "{}", val.1.info()).unwrap();
        }
        info
    }
    pub fn device_status(
        &self,
        room_name: &str,
        device_name: &str,
    ) -> Result<String, SmartHouseErrors> {
        let room = self.rooms.get(room_name);
        let mut info = String::new();
        match room {
            Some(room) => {
                let device = room.devices.get(device_name);
                match device {
                    Some(device) => match device {
                        CleverDevice::ElecticSocket(electric_socket) => write!(
                            info,
                            "Room {} Device status {}",
                            room_name,
                            electric_socket.info()
                        )
                        .unwrap(),
                        CleverDevice::Thermometer(thermometer) => write!(
                            info,
                            "Room {} Device status {}",
                            room_name,
                            thermometer.info()
                        )
                        .unwrap(),
                    },
                    None => {
                        return Err(SmartHouseErrors::DeviceNotFound((
                            room_name.to_string(),
                            device_name.to_string(),
                        )))
                    }
                }
            }
            None => return Err(SmartHouseErrors::RoomNotFound(room_name.to_string())),
        }
        Ok(info)
    }
}
