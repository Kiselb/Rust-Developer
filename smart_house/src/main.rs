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
    let mut house = SmartHouse::new(String::from(
        "Smart House ================================================================",
    ));

    let mut room1 = SmartRoom::new(String::from("Room1"));

    let es = Box::new(ElectricSocket::new(String::from("Electric socket #1")));
    room1.add(es);
    let es = Box::new(ElectricSocket::new(String::from("Electric socket #2")));
    room1.add(es);
    let mut es = Box::new(ElectricSocket::new(String::from("Electric socket #3")));
    es.on();
    room1.add(es);
    let es = Box::new(ElectricSocket::new(String::from("Electric socket #4")));
    room1.add(es);

    let t = Box::new(Thermometer::new(String::from("Thermometer #1")));
    room1.add(t);

    house.add(room1);

    let mut room2 = SmartRoom::new(String::from("Room2"));
    let es = Box::new(ElectricSocket::new(String::from("Electric socket #1")));
    room2.add(es);
    let es = Box::new(ElectricSocket::new(String::from("Electric socket #2")));
    room2.add(es);
    let es = Box::new(ElectricSocket::new(String::from("Electric socket #3")));
    room2.add(es);
    let mut es = Box::new(ElectricSocket::new(String::from("Electric socket #4")));
    es.on();
    room2.add(es);

    let t = Box::new(Thermometer::new(String::from("Thermometer #1")));
    room2.add(t);

    house.add(room2);
    println!("{}", house.info());

    house.device_status("Room1", "Electric socket #4");
    println!();
}

fn with_clever_room() {
    let mut house = CleverHouse::new(String::from(
        "Clever House ================================================================",
    ));

    let mut room1 = CleverRoom::new(String::from("Room1"));

    let es = CleverDevice::ElecticSocket(ElectricSocket::new(String::from("Electric socket #1")));
    room1.add(es);
    let es = CleverDevice::ElecticSocket(ElectricSocket::new(String::from("Electric socket #2")));
    room1.add(es);
    let mut es = ElectricSocket::new(String::from("Electric socket #3"));
    es.on();
    room1.add(CleverDevice::ElecticSocket(es));
    let es = CleverDevice::ElecticSocket(ElectricSocket::new(String::from("Electric socket #4")));
    room1.add(es);

    let t = CleverDevice::Thermometer(Thermometer::new(String::from("Thermometer #1")));
    room1.add(t);

    house.add(room1);

    let mut room2 = CleverRoom::new(String::from("Room2"));
    let es = CleverDevice::ElecticSocket(ElectricSocket::new(String::from("Electric socket #1")));
    room2.add(es);
    let es = CleverDevice::ElecticSocket(ElectricSocket::new(String::from("Electric socket #2")));
    room2.add(es);
    let es = CleverDevice::ElecticSocket(ElectricSocket::new(String::from("Electric socket #3")));
    room2.add(es);
    let mut es = ElectricSocket::new(String::from("Electric socket #4"));
    es.on();
    room2.add(CleverDevice::ElecticSocket(es));

    let t = CleverDevice::Thermometer(Thermometer::new(String::from("Thermometer #1")));
    room2.add(t);

    house.add(room2);
    house.info();

    house.device_status("Room1", "Electric socket #4");
    println!();
}
