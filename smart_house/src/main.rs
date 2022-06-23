use core::panic;

use smart_house_lib::clever_house::CleverHouse;
use smart_house_lib::smart_house::clever_room::clever_device::CleverDevice;
use smart_house_lib::smart_house::clever_room::CleverRoom;
use smart_house_lib::smart_house::smart_room::electric_socket::ElectricSocket;
use smart_house_lib::smart_house::smart_room::thermometer::Thermometer;
use smart_house_lib::smart_house::smart_room::SmartRoom;
use smart_house_lib::smart_house::SmartHouse;

fn main() {
    with_smart_room();
    with_clever_room();
}

fn with_smart_room() {
    let mut house = match SmartHouse::new(String::from("Smart House #1")) {
        Ok(house) => house,
        Err(e) => panic!("{:?}", e),
    };

    // Room #1

    let mut room1 = match SmartRoom::new(String::from("Smart Room #1")) {
        Ok(room) => room,
        Err(e) => panic!("{:?}", e),
    };

    let es = Box::new(
        match ElectricSocket::new(String::from("Electric socket #1")) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room1.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };
    let es = Box::new(
        match ElectricSocket::new(String::from("Electric socket #2")) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room1.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };
    let mut es = Box::new(
        match ElectricSocket::new(String::from("Electric socket #3")) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    es.on();
    match room1.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };
    let es = Box::new(
        match ElectricSocket::new(String::from("Electric socket #4")) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room1.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    let t = Box::new(match Thermometer::new(String::from("Thermometer #1")) {
        Ok(device) => device,
        Err(e) => panic!("{:?}", e),
    });
    match room1.add(t) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    match house.add(room1) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    // Room #2

    let mut room2 = match SmartRoom::new(String::from("Smart Room #2")) {
        Ok(room) => room,
        Err(e) => panic!("{:?}", e),
    };

    let es = Box::new(
        match ElectricSocket::new(String::from("Electric socket #1")) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room2.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };
    let es = Box::new(
        match ElectricSocket::new(String::from("Electric socket #2")) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room2.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };
    let es = Box::new(
        match ElectricSocket::new(String::from("Electric socket #3")) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room2.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };
    let mut es = Box::new(
        match ElectricSocket::new(String::from("Electric socket #4")) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    es.on();
    match room2.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    let t = Box::new(match Thermometer::new(String::from("Thermometer #1")) {
        Ok(device) => device,
        Err(e) => panic!("{:?}", e),
    });
    match room2.add(t) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    match house.add(room2) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    println!("{}", house.info());

    match house.device_status("Smart Room #1", "Electric socket #4") {
        Ok(info) => println!("{}", info),
        Err(e) => panic!("{:?}", e),
    };
    println!();
}

fn with_clever_room() {
    let mut house = match CleverHouse::new(String::from("Clever House #1")) {
        Ok(house) => house,
        Err(e) => panic!("{:?}", e),
    };

    let mut room1 = match CleverRoom::new(String::from("Clever Room #1")) {
        Ok(room) => room,
        Err(e) => panic!("{:?}", e),
    };

    let es = CleverDevice::ElecticSocket(
        match ElectricSocket::new(String::from("Electric socket #1")) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room1.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    let es = CleverDevice::ElecticSocket(
        match ElectricSocket::new(String::from("Electric socket #2")) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room1.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    let es = CleverDevice::ElecticSocket(
        match ElectricSocket::new(String::from("Electric socket #3")) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room1.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };
    // Получение &mut на устройство в коллекции
    //
    // Вариант 1
    //
    // match room1.get_mut("Electric socket #3") {
    //     Some(device) => match device  {
    //         CleverDevice::ElecticSocket(electric_socket) => electric_socket.on(),
    //         _ => ()
    //     }
    //     None => ()
    // };
    //
    // Вариант 2
    //
    // match room1.get_mut("Electric socket #3") {
    //     Some(CleverDevice::ElecticSocket(electric_socket)) => electric_socket.on(),
    //     _ => ()
    // }
    //
    // Вариант итоговый
    //
    if let Some(CleverDevice::ElecticSocket(electric_socket)) = room1.get_mut("Electric socket #3")
    {
        electric_socket.on()
    }

    let es = CleverDevice::ElecticSocket(
        match ElectricSocket::new(String::from("Electric socket #4")) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room1.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    let es = CleverDevice::Thermometer(match Thermometer::new(String::from("Thermometer #1")) {
        Ok(device) => device,
        Err(e) => panic!("{:?}", e),
    });
    match room1.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };
    match house.add(room1) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    println!("{}", house.info());

    match house.device_status("Clever Room #1", "Electric socket #3") {
        Ok(info) => println!("{}", info),
        Err(e) => panic!("{:?}", e),
    };
    println!();
}
