use std::fmt::Write;
use std::str;
use std::{
    error::Error,
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
    thread,
};

use crate::results::FrameError;

pub mod results;

pub const SDCPU_PACKET_HEADER: &str = "SDCPU 0.0.1";

pub struct ParamItem {
    pub name: String,
    pub value: String,
}

impl ParamItem {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

pub struct SdcpuFrame {
    pub protocol: String,
    pub parameters: Vec<ParamItem>,
}

pub type SdcpuFrameShared = Arc<Mutex<Box<SdcpuFrame>>>;

pub struct SdcpuHandler;

impl SdcpuHandler {
    pub fn new(
        address: SocketAddr,
        shared_frame: SdcpuFrameShared,
    ) -> Result<Self, Box<dyn Error>> {
        thread::spawn(move || {
            let socket = UdpSocket::bind(address).unwrap();
            loop {
                let mut datagram = [0; 1024];
                match socket.recv_from(&mut datagram) {
                    Ok(response) => {
                        **shared_frame.lock().unwrap() =
                            make_frame(&datagram, response.0).unwrap_or(SdcpuFrame {
                                protocol: SDCPU_PACKET_HEADER.to_string(),
                                parameters: vec![],
                            });
                    }
                    Err(error) => panic!("Datagram receiving error: {error}"),
                }
            }
        });
        Ok(Self)
    }
}

pub fn make_frame(datagram: &[u8], length: usize) -> Result<SdcpuFrame, FrameError> {
    let mut frame: SdcpuFrame = SdcpuFrame {
        protocol: SDCPU_PACKET_HEADER.to_string(),
        parameters: vec![],
    };
    let frame_bytes = &datagram[..length];
    let raw_frame = str::from_utf8(frame_bytes)?;

    frame.parameters = raw_frame
        .split(';')
        .filter(|&item| item.contains('='))
        .map(|pair| {
            ParamItem::new(
                (&pair[..pair.find('=').unwrap()])
                    .to_string()
                    .to_uppercase(),
                (&pair[(pair.find('=').unwrap() + 1)..])
                    .to_string()
                    .to_uppercase(),
            )
        })
        .collect();
    match frame.parameters.iter().find(|&item| item.name.eq("HEADER")) {
        Some(protocol) => frame.protocol = protocol.value.to_string(),
        None => (),
    }
    Ok(frame)
}

pub fn make_packet(frame: &SdcpuFrame) -> String {
    let mut data: String = String::new();

    write!(data, "HEADER={};", frame.protocol).unwrap();
    for parameter in frame.parameters.iter() {
        write!(data, "{}={};", parameter.name, parameter.value).unwrap();
    }

    data
}
