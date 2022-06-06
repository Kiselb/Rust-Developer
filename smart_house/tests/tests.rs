use smart_house_lib::smart_house::{
    smart_room::{
        electric_socket::ElectricSocket, smart_device::SmartDevice, thermometer::Thermometer,
        SmartRoom,
    },
    SmartHouse,
};

#[test]
fn test_house_report() {
    let value = 5;
    assert_eq!(5, value);
}
#[test]
fn test_electric_socket_off_report() {
    let device = Box::new(ElectricSocket::new(String::from("Electric socket #1")));
    let device_info = device.info();
    assert_eq!(
        "Electric socket: Electric socket #1 State: OFF",
        device_info
    );
}
#[test]
fn test_electric_socket_on_report() {
    let mut device = Box::new(ElectricSocket::new(String::from("Electric socket #1")));
    device.on();
    let device_info = device.info();
    assert_eq!(
        "Electric socket: Electric socket #1 State: ON Consumption power: 0",
        device_info
    );
}
#[test]
fn test_thermometer_initial_report() {
    let device = Box::new(Thermometer::new(String::from("Thermometer #1")));
    let device_info = device.info();
    assert_eq!("Thermometer: Thermometer #1 Value: 0", device_info);
}
#[test]
fn test_thermometer_onaction_report() {
    let mut device = Box::new(Thermometer::new(String::from("Thermometer #1")));
    device.set_temperature(25);
    let device_info = device.info();
    assert_eq!("Thermometer: Thermometer #1 Value: 25", device_info);
}
#[test]
fn test_smart_room_init_report() {
    let room = SmartRoom::new(String::from("Room #1"));
    let room_info = room.info();
    assert_eq!("", room_info);
}
#[test]
fn test_smart_room_onaction_report() {
    let mut room = SmartRoom::new(String::from("Room #1"));

    let device = Box::new(ElectricSocket::new(String::from("Electric socket #1")));
    room.add(device);

    let device = Box::new(Thermometer::new(String::from("Thermometer #1")));
    room.add(device);

    let room_info = room.info();
    assert_eq!(
        "Electric socket: Electric socket #1 State: OFF\nThermometer: Thermometer #1 Value: 0\n",
        room_info
    );
}
#[test]
fn test_smart_house_init_report() {
    let house = SmartHouse::new(String::from("Smart House"));
    let info = house.info();
    assert_eq!("House Smart House\n", info);
}
#[test]
fn test_smart_house_onaction_report() {
    let mut house = SmartHouse::new(String::from("Smart House"));

    let mut room1 = SmartRoom::new(String::from("Room #1"));

    let device = Box::new(ElectricSocket::new(String::from("Electric socket #1")));
    room1.add(device);

    let device = Box::new(Thermometer::new(String::from("Thermometer #1")));
    room1.add(device);

    house.add(room1);

    let mut room2 = SmartRoom::new(String::from("Room #2"));

    let device = Box::new(ElectricSocket::new(String::from("Electric socket #1")));
    room2.add(device);

    let device = Box::new(Thermometer::new(String::from("Thermometer #1")));
    room2.add(device);

    house.add(room2);

    let info = house.info();
    assert_eq!("House Smart House\nRoom: Room #1\nElectric socket: Electric socket #1 State: OFF\nThermometer: Thermometer #1 Value: 0\nRoom: Room #2\nElectric socket: Electric socket #1 State: OFF\nThermometer: Thermometer #1 Value: 0\n", info);
}
