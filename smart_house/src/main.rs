struct _ElectricSocket {
    name: String,
    power_consumption: u32,
    status: bool,
}
struct _Thermometer {
    name: String,
    temperature: i8,
}
impl _ElectricSocket {
    pub fn _new(name: String) -> Self {
        Self {
            name,
            power_consumption: 0,
            status: false,
        }
    }
    pub fn _on(&mut self) {
        todo!()
    }
    pub fn _off(&mut self) {
        todo!()
    }
    pub fn _name(&self) -> String {
        todo!()
    }
    pub fn _power(&self) -> u32 {
        todo!()
    }
    pub fn _status(&self) -> bool {
        todo!()
    }
}
impl _Thermometer {
    pub fn _new(name: String) -> Self {
        Self {
            name,
            temperature: 0,
        }
    }
    pub fn _name(&self) -> String {
        todo!()
    }
    pub fn _value(&self) -> i8 {
        todo!()
    }
}
fn main() {
    todo!()
}
