use core::panic;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use sdcp::{NetConfig, ParamItem, SdcpFrame};
use smart_house_lib::clever_house::CleverHouse;
use smart_house_lib::smart_house::clever_room::clever_device::CleverDevice;
use smart_house_lib::smart_house::clever_room::CleverRoom;
use smart_house_lib::smart_house::smart_room::electric_socket::ElectricSocket;
use smart_house_lib::smart_house::smart_room::thermometer::Thermometer;
use smart_house_lib::smart_house::smart_room::SmartRoom;
use smart_house_lib::smart_house::SmartHouse;

use sdcp::{SdcpHandler, SDCP_COMMANDS, SDCP_OK, SDCP_PACKET_HEADER, SDCP_PARAM_STATUS};

fn main() {
    // with_smart_room();
    // with_clever_room();

    println!("Server started");
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 55000);

    let handler = SdcpHandler::new(address);
    let es_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 55100);

    //let parameters = vec![ParamItem::new( SDCP_PARAM_STATUS.to_string(), "".to_string()), ParamItem::new( SDCP_PARAM_PWRCON.to_string(), "".to_string())];
    let parameters = vec![ParamItem::new(
        SDCP_PARAM_STATUS.to_string(),
        "true".to_string(),
    )];

    let frame: SdcpFrame = SdcpFrame {
        protocol: SDCP_PACKET_HEADER.to_string(),
        command: SDCP_COMMANDS::SETP.to_string(),
        parameters,
        result: SDCP_OK.to_string(),
    };
    match handler.request(frame, es_address) {
        Ok(frame) => {
            println!("Request completed successfully");
            println!("Protocol: {};", frame.protocol);
            println!("Command: {};", frame.command);
            for item in frame.parameters.iter() {
                println!("Parameter: {}={};", item.name, item.value);
            }
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }

    loop {
        let exit = String::from("exit");
        let mut buffer: String = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        if buffer.trim().eq(&exit) {
            std::process::exit(0);
        }
        let tokens = buffer.trim().split(' ').collect::<Vec<&str>>();
        if !tokens.is_empty() {
            let mut frame: SdcpFrame = SdcpFrame {
                protocol: SDCP_PACKET_HEADER.to_string(),
                command: SDCP_COMMANDS::NONE.to_string(),
                parameters: vec![],
                result: SDCP_OK.to_string(),
            };
            if "SET".to_uppercase().eq(&tokens[0].to_uppercase()) && tokens.len() == 3 {
                frame.command = SDCP_COMMANDS::SETP.to_string();
                frame
                    .parameters
                    .push(ParamItem::new(tokens[1].to_string(), tokens[2].to_string()))
            } else if "GET".to_uppercase().eq(&tokens[0].to_uppercase()) && tokens.len() == 2 {
                frame.command = SDCP_COMMANDS::GETP.to_string();
                frame
                    .parameters
                    .push(ParamItem::new(tokens[1].to_string(), "".to_string()))
            } else {
                println!("Invalid command");
                continue;
            }
            match handler.request(frame, es_address) {
                Ok(frame) => {
                    println!("Request completed successfully");
                    println!("Protocol: {};", frame.protocol);
                    println!("Command: {};", frame.command);
                    for item in frame.parameters.iter() {
                        println!("Parameter: {}={};", item.name, item.value);
                    }
                }
                Err(error) => {
                    println!("Error: {}", error);
                }
            }
        }
    }
}

fn _with_smart_room() {
    let mut house = match SmartHouse::new(String::from("Smart House #1")) {
        Ok(house) => house,
        Err(e) => panic!("{:?}", e),
    };

    // Room #1

    let mut room1 = match SmartRoom::new(String::from("Smart Room #1")) {
        Ok(room) => room,
        Err(e) => panic!("{:?}", e),
    };

    let net_config = NetConfig::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        55000,
    ));

    let es = Box::new(
        match ElectricSocket::new(String::from("Electric socket #1"), net_config) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room1.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    let net_config = NetConfig::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        55000,
    ));

    let es = Box::new(
        match ElectricSocket::new(String::from("Electric socket #2"), net_config) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room1.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    let net_config = NetConfig::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        55000,
    ));

    let mut es = Box::new(
        match ElectricSocket::new(String::from("Electric socket #3"), net_config) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    es.on();
    match room1.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    let net_config = NetConfig::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        55000,
    ));

    let es = Box::new(
        match ElectricSocket::new(String::from("Electric socket #4"), net_config) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room1.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    let net_config = NetConfig::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        55000,
    ));

    let t = Box::new(
        match Thermometer::new(String::from("Thermometer #1"), net_config) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
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

    let net_config = NetConfig::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        55000,
    ));

    let es = Box::new(
        match ElectricSocket::new(String::from("Electric socket #1"), net_config) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room2.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    let net_config = NetConfig::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        55000,
    ));

    let es = Box::new(
        match ElectricSocket::new(String::from("Electric socket #2"), net_config) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room2.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    let net_config = NetConfig::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        55000,
    ));

    let es = Box::new(
        match ElectricSocket::new(String::from("Electric socket #3"), net_config) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room2.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    let net_config = NetConfig::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        55000,
    ));

    let mut es = Box::new(
        match ElectricSocket::new(String::from("Electric socket #4"), net_config) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    es.on();
    match room2.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    let net_config = NetConfig::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        55000,
    ));

    let t = Box::new(
        match Thermometer::new(String::from("Thermometer #1"), net_config) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
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

fn _with_clever_room() {
    let mut house = match CleverHouse::new(String::from("Clever House #1")) {
        Ok(house) => house,
        Err(e) => panic!("{:?}", e),
    };

    let mut room1 = match CleverRoom::new(String::from("Clever Room #1")) {
        Ok(room) => room,
        Err(e) => panic!("{:?}", e),
    };

    let net_config = NetConfig::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        55000,
    ));

    let es = CleverDevice::ElecticSocket(
        match ElectricSocket::new(String::from("Electric socket #1"), net_config) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room1.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    let net_config = NetConfig::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        55000,
    ));

    let es = CleverDevice::ElecticSocket(
        match ElectricSocket::new(String::from("Electric socket #2"), net_config) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room1.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    let net_config = NetConfig::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        55000,
    ));

    let es = CleverDevice::ElecticSocket(
        match ElectricSocket::new(String::from("Electric socket #3"), net_config) {
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

    let net_config = NetConfig::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        55000,
    ));

    let es = CleverDevice::ElecticSocket(
        match ElectricSocket::new(String::from("Electric socket #4"), net_config) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
    match room1.add(es) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };

    let net_config = NetConfig::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        55000,
    ));

    let es = CleverDevice::Thermometer(
        match Thermometer::new(String::from("Thermometer #1"), net_config) {
            Ok(device) => device,
            Err(e) => panic!("{:?}", e),
        },
    );
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
