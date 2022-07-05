use std::collections::HashMap;
use std::fmt::Write;

pub mod clever_room;
pub mod errors;
pub mod smart_room;

use crate::smart_house::errors::SmartHouseErrors;
use crate::smart_house::smart_room::SmartRoom;
use crate::smart_house::smart_room::SMART_ROOM_NAME_MIN_LENGTH;

pub const SMART_HOUSE_NAME_MIN_LENGTH: usize = 8;

pub struct SmartHouse {
    name: String,
    rooms: HashMap<String, SmartRoom>,
}
impl SmartHouse {
    pub fn new(name: String) -> Result<Self, SmartHouseErrors> {
        if name.len() < SMART_HOUSE_NAME_MIN_LENGTH {
            return Err(SmartHouseErrors::InvalidHouseName);
        };
        Ok(Self {
            name,
            rooms: HashMap::new(),
        })
    }

    pub fn add(&mut self, room: SmartRoom) -> Result<(), SmartHouseErrors> {
        let room_name = String::from(&room.name);
        if room_name.len() < SMART_ROOM_NAME_MIN_LENGTH {
            return Err(SmartHouseErrors::InvalidRoomName);
        }
        self.rooms.insert(room_name, room);
        Ok(())
    }

    pub fn rem(&mut self, room_name: &str) -> Option<SmartRoom> {
        self.rooms.remove_entry(room_name).map(|(_, room)| room)
    }

    pub fn get(&self, room_name: &str) -> Option<&SmartRoom> {
        self.rooms.get(room_name)
    }

    pub fn get_mut(&mut self, room_name: &str) -> Option<&mut SmartRoom> {
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
        writeln!(info, "House {}", &self.name).unwrap();
        let mut rooms: Vec<_> = self.rooms.iter().collect();
        rooms.sort_by(|op1, op2| op1.0.cmp(op2.0));
        for val in rooms {
            writeln!(info, "Room: {}", val.0).unwrap();
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
                    Some(device) => {
                        writeln!(info, "Room {} Device status {}", room_name, device.info())
                            .unwrap()
                    }
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
