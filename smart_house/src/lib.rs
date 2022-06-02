use std::collections::HashMap;

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
pub struct SmartRoom {
    name: String,
    devices: HashMap<String, Box<dyn SmartDevice>>,
}
impl SmartRoom {
    pub fn new(name: String) -> Self {
        Self {
            name,
            devices: HashMap::new(),
        }
    }
    pub fn info(&self) {
        println!("Room: {}", &self.name);
        for val in &mut self.devices.values() {
            println!("{}", val.info());
        }
    }
    pub fn add(&mut self, device: Box<dyn SmartDevice>) {
        self.devices.insert(String::from(device.identity()), device);
    }
    //#[allow(clippy::borrowed_box)]
    pub fn get(&self, device_name: &str) -> Option<&dyn SmartDevice> {
        self.devices.get(device_name).map(|v| v.as_ref())
    }
}
pub trait SmartDevice {
    fn identity(&self) -> &String;
    fn info(&self) -> String;
}
/*
    Electric Socket
*/
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
        let mut info = String::from("Electric socket: ");
        info.push_str(&self.name);
        if self.status {
            info.push_str(" State: ON");
            info.push_str(" Consumption power: ");
            info.push_str(&self.power_consumption.to_string());
        } else {
            info.push_str(" State: OFF");
        }
        info
    }
}
impl ElectricSocket {
    pub fn new(name: String) -> Self {
        Self {
            name,
            power_consumption: 0,
            status: false,
        }
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
/*
   Thermometer
*/
pub struct Thermometer {
    name: String,
    temperature: i8,
}
impl SmartDevice for Thermometer {
    fn identity(&self) -> &String {
        &self.name
    }
    fn info(&self) -> String {
        let mut info = String::from("Thermometer: ");
        info.push_str(&self.name);
        info.push_str(" Value: ");
        info.push_str(&self.temperature.to_string());
        info
    }
}
impl Thermometer {
    pub fn new(name: String) -> Self {
        Self {
            name,
            temperature: 0,
        }
    }
    pub fn temperature(&self) -> i8 {
        self.temperature
    }
}
