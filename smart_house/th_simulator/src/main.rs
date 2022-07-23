use std::io::{self, Write};
use std::thread;
use std::{
    net::{SocketAddr, UdpSocket},
    time::Duration,
};

use sdcpu::{make_packet, ParamItem, SdcpuFrame, SDCPU_PACKET_HEADER};

use th_simulator::TH_PARAM_TEMPERATURE;

fn main() {
    let mut args = std::env::args();
    let address = args.nth(1).expect("Не задан целевой адрес");
    println!("Адрес приёмника: {}", address);

    let target = address.parse::<SocketAddr>().unwrap();

    let address = "127.0.0.1:4000";
    let socket = UdpSocket::bind(address)
        .expect("Ошибка привязки к адресу, возможно требуемый порт уже занят");
    let mut i = 10.0;

    loop {
        i += 0.25;
        let frame: SdcpuFrame = SdcpuFrame {
            protocol: SDCPU_PACKET_HEADER.to_string(),
            parameters: vec![ParamItem::new(
                TH_PARAM_TEMPERATURE.to_string(),
                i.to_string(),
            )],
        };
        let packet = make_packet(&frame);
        let data = packet.as_bytes();
        let result = socket.send_to(data, target);
        match result {
            Ok(_) => {
                print!("Температура устройства {i}\r");
                io::stdout().flush().unwrap();
            }
            Err(error) => println!("Ошибка отправки UDP пакета: {error}"),
        }
        thread::sleep(Duration::from_secs(1));
    }
}
