use smart_house_lib::smart_house::{
    smart_room::{
        electric_socket::ElectricSocket, smart_device::SmartDevice, thermometer::Thermometer,
        SmartRoom,
    },
    SmartHouse,
};

#[test]
fn test_electric_socket_off_report() {
    let device = Box::new(
        match ElectricSocket::new(String::from("Electric socket #1")) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    let device_info = device.info();
    assert_eq!(
        "Electric socket: Electric socket #1 State: OFF",
        device_info
    );
}
#[test]
fn test_electric_socket_on_report() {
    let mut device = Box::new(
        match ElectricSocket::new(String::from("Electric socket #1")) {
            Ok(mut device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    device.on();
    let device_info = device.info();
    assert_eq!(
        "Electric socket: Electric socket #1 State: ON Consumption power: 0",
        device_info
    );
}
#[test]
fn test_thermometer_initial_report() {
    let device = Box::new(match Thermometer::new(String::from("Thermometer #1")) {
        Ok(device) => device,
        Err(e) => panic!("{:?}", e),
    });
    let device_info = device.info();
    assert_eq!("Thermometer: Thermometer #1 Value: 0", device_info);
}
#[test]
fn test_thermometer_onaction_report() {
    let mut device = Box::new(match Thermometer::new(String::from("Thermometer #1")) {
        Ok(mut device) => device,
        Err(e) => panic!("{:?}", e),
    });
    device.set_temperature(25);
    let device_info = device.info();
    assert_eq!("Thermometer: Thermometer #1 Value: 25", device_info);
}
#[test]
fn test_smart_room_init_report() {
    let room = match SmartRoom::new(String::from("Smart Room #1")) {
        Ok(room) => room,
        Err(e) => panic!("{:?}", e),
    };
    let room_info = room.info();
    assert_eq!("", room_info);
}
#[test]
fn test_smart_room_onaction_report() {
    let mut room = match SmartRoom::new(String::from("Smart Room #1")) {
        Ok(room) => room,
        Err(e) => panic!("{:?}", e),
    };

    let device = Box::new(
        match ElectricSocket::new(String::from("Electric socket #1")) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    room.add(device);

    let device = Box::new(match Thermometer::new(String::from("Thermometer #1")) {
        Ok(device) => device,
        Err(e) => panic!("{:?}", e),
    });
    room.add(device);

    let room_info = room.info();
    assert_eq!(
        "Electric socket: Electric socket #1 State: OFF\nThermometer: Thermometer #1 Value: 0\n",
        room_info
    );
}
#[test]
fn test_smart_house_init_report() {
    let house = match SmartHouse::new(String::from("Smart House")) {
        Ok(house) => house,
        Err(e) => panic!("{:?}", e),
    };
    let info = house.info();
    assert_eq!("House Smart House\n", info);
}
#[test]
fn test_smart_house_onaction_report() {
    let mut house = match SmartHouse::new(String::from("Smart House")) {
        Ok(house) => house,
        Err(e) => panic!("{:?}", e),
    };

    let mut room1 = match SmartRoom::new(String::from("Smart Room #1")) {
        Ok(room) => room,
        Err(e) => panic!("{:?}", e),
    };

    let device = Box::new(
        match ElectricSocket::new(String::from("Electric socket #1")) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    room1.add(device);

    let device = Box::new(match Thermometer::new(String::from("Thermometer #1")) {
        Ok(device) => device,
        Err(e) => panic!("{:?}", e),
    });
    room1.add(device);

    let mut room2 = match SmartRoom::new(String::from("Smart Room #2")) {
        Ok(room) => room,
        Err(e) => panic!("{:?}", e),
    };

    house.add(room1);

    let device = Box::new(
        match ElectricSocket::new(String::from("Electric socket #1")) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    room2.add(device);

    let device = Box::new(match Thermometer::new(String::from("Thermometer #1")) {
        Ok(device) => device,
        Err(e) => panic!("{:?}", e),
    });
    room2.add(device);

    house.add(room2);

    let info = house.info();
    assert_eq!("House Smart House\nRoom: Smart Room #1\nElectric socket: Electric socket #1 State: OFF\nThermometer: Thermometer #1 Value: 0\nRoom: Smart Room #2\nElectric socket: Electric socket #1 State: OFF\nThermometer: Thermometer #1 Value: 0\n", info);
}
