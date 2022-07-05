#[derive(Debug)]
pub enum SmartHouseErrors {
    InvalidHouseName,
    InvalidRoomName,
    InvalidDeviceIdentity,
    RoomNotFound(String),
    DeviceNotFound((String, String)),
}
