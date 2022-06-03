use std::collections::HashMap;
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
    pub fn info(&self) {
        println!("House '{}'", &self.name);
        for val in &mut self.rooms.values() {
            val.info();
        }
    }
    pub fn device_status(&self, room_name: &str, device_name: &str) {
        let room = self.rooms.get(room_name);
        match room {
            Some(room) => {
                let device = room.devices.get(device_name);
                match device {
                    Some(device) => {
                        println!("Room {} Device status {}", room_name, device.info());
                    }
                    None => println!(
                        "Device {} in Room {} does not exists",
                        device_name, room_name
                    ),
                }
            }
            None => println!("Room {} does not exists", room_name),
        }
    }
}
