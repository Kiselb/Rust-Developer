use std::collections::HashMap;
use std::fmt::Write;

pub mod clever_room;
pub mod smart_room;

use crate::smart_house::smart_room::SmartRoom;

pub struct SmartHouse {
    name: String,
    rooms: HashMap<String, SmartRoom>,
}
impl SmartHouse {
    pub fn new(name: String) -> Self {
        Self {
            name,
            rooms: HashMap::new(),
        }
    }
    pub fn add(&mut self, room: SmartRoom) {
        self.rooms.insert(String::from(&room.name), room);
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
    pub fn device_status(&self, room_name: &str, device_name: &str) -> String {
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
                    None => writeln!(
                        info,
                        "Device {} in Room {} does not exists",
                        device_name, room_name
                    )
                    .unwrap(),
                }
            }
            None => writeln!(info, "Room {} does not exists", room_name).unwrap(),
        }
        info
    }
}
