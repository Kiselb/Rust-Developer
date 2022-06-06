use std::collections::HashMap;
use std::fmt::Write;

use crate::smart_house::clever_room::clever_device::CleverDevice;
use crate::smart_house::clever_room::CleverRoom;
use crate::smart_house::smart_room::smart_device::SmartDevice;

pub struct CleverHouse {
    name: String,
    rooms: HashMap<String, CleverRoom>,
}
impl CleverHouse {
    pub fn new(name: String) -> Self {
        Self {
            name,
            rooms: HashMap::new(),
        }
    }
    pub fn add(&mut self, room: CleverRoom) {
        self.rooms.insert(String::from(&room.name), room);
    }
    pub fn info(&self) {
        let mut info = String::new();
        writeln!(info, "House '{}'", &self.name).unwrap();
        let mut rooms: Vec<_> = self.rooms.iter().collect();
        rooms.sort_by(|op1, op2| op1.0.cmp(op2.0));
        for val in rooms {
            write!(info, "{}", val.1.info()).unwrap();
        }
    }
    pub fn device_status(&self, room_name: &str, device_name: &str) -> String {
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
                    None => write!(
                        info,
                        "Device {} in Room {} does not exists",
                        device_name, room_name
                    )
                    .unwrap(),
                }
            }
            None => write!(info, "Room {} does not exists", room_name).unwrap(),
        }
        info
    }
}
