use smart_house_lib::smart_house::SmartHouse;
use smart_house_lib::smart_house::smart_room::SmartRoom;
use smart_house_lib::smart_house::smart_room::electric_socket::ElectricSocket;
use smart_house_lib::smart_house::smart_room::thermometer::Thermometer;

fn main() {
    let mut house = SmartHouse::new(String::from("Smart House"));

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
    house.info();

    house.device_status("Room1", "Electric socket #4");
}
