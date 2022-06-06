use crate::smart_house::smart_room::electric_socket::ElectricSocket;
use crate::smart_house::smart_room::thermometer::Thermometer;

pub enum CleverDevice {
    ElecticSocket(ElectricSocket),
    Thermometer(Thermometer),
}
