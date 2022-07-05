use std::fmt::Write;
use std::io::{Read as IORead, Write as IOWrite};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::str;
use std::thread;

use crate::results::{
    FrameError, FrameResult, RecvError, RecvResult, RequestError, RequestResult, SendResult,
};

pub mod results;

pub const SDCP_PACKET_HEADER: &str = "SDCP 0.0.1";

const SDCP_COMMAND_SETP: &str = "SETP";
const SDCP_COMMAND_GETP: &str = "GETP";
const SDCP_COMMAND_BEAT: &str = "BEAT";
const SDCP_COMMAND_INFO: &str = "INFO";
const SDCP_COMMAND_NONE: &str = "NONE";

#[allow(non_camel_case_types)]
pub struct SDCP_COMMANDS;

impl SDCP_COMMANDS {
    pub const SETP: &'static str = SDCP_COMMAND_SETP;
    pub const GETP: &'static str = SDCP_COMMAND_GETP;
    pub const BEAT: &'static str = SDCP_COMMAND_BEAT;
    pub const INFO: &'static str = SDCP_COMMAND_INFO;
    pub const NONE: &'static str = SDCP_COMMAND_NONE;
}

pub const SDCP_PARAM_STATUS: &str = "STATUS";
pub const SDCP_PARAM_PWRCON: &str = "PWRCON";

pub const SDCP_OK: &str = "OK";
pub const SDCP_FAILED: &str = "FAILED";

pub struct ParamItem {
    pub name: String,
    pub value: String,
}

impl ParamItem {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

pub type SdcpRequestHandler = fn(FrameResult) -> SdcpFrame;

pub struct SdcpFrame {
    pub protocol: String,
    pub command: String,
    pub parameters: Vec<ParamItem>,
    pub result: String,
}

pub struct NetConfig {
    pub net_address: SocketAddr,
}

impl NetConfig {
    pub fn new(net_address: SocketAddr) -> Self {
        Self { net_address }
    }
}

pub struct SdcpHandler {
    address: SocketAddr,
}

impl SdcpHandler {
    pub fn new(address: SocketAddr) -> Self {
        Self { address }
    }
    pub fn bind(&self, handler: SdcpRequestHandler) {
        let address = self.address;
        thread::spawn(move || {
            let listener = TcpListener::bind(address).unwrap();
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => match recv_packet(&stream) {
                        Ok(packet) => match stream.peer_addr() {
                            Ok(address) => {
                                println!("Source address: {}", address);
                                match send_packet(make_packet(handler(make_frame(packet))), &stream)
                                {
                                    Ok(()) => (),
                                    Err(error) => println!("Packet sending error: {}", error),
                                }
                            }
                            Err(error) => println!("Error getting address: {}", error),
                        },
                        Err(error) => println!("Error receiving package: {}", error),
                    },
                    Err(error) => println!("Connection error: {}", error),
                }
            }
        });
    }

    pub fn request(&self, frame: SdcpFrame, address: SocketAddr) -> RequestResult {
        match TcpStream::connect(address) {
            Ok(stream) => match send_packet(make_packet(frame), &stream) {
                Ok(_) => match recv_packet(&stream) {
                    Ok(packet) => match make_frame(packet) {
                        Ok(frame) => Ok(frame),
                        Err(_) => Err(RequestError::InvalidPacket),
                    },
                    Err(error) => Err(RequestError::Recv(error)),
                },
                Err(error) => Err(RequestError::Send(error)),
            },
            Err(error) => Err(RequestError::Io(error)),
        }
    }
}

pub fn make_frame(data: String) -> FrameResult {
    let mut frame: SdcpFrame = SdcpFrame {
        protocol: SDCP_PACKET_HEADER.to_string(),
        command: SDCP_COMMAND_NONE.to_string(),
        parameters: vec![],
        result: SDCP_OK.to_string(),
    };
    for item in data.split(';').collect::<Vec<&str>>().iter() {
        if !item.is_empty() {
            // exclude tail
            let pair = item.split('=').collect::<Vec<&str>>();
            if pair.len() != 2 {
                return Err(FrameError::InvalidPacket);
            } else if "Command".to_uppercase().eq(&pair[0].to_uppercase()) && !pair[1].is_empty() {
                frame.command = pair[1].to_string();
            } else if !"Result".to_uppercase().eq(&pair[0].to_uppercase()) {
                frame
                    .parameters
                    .push(ParamItem::new(pair[0].to_string(), pair[1].to_string()));
            }
        }
    }
    if !frame.parameters.is_empty() {
        frame
            .parameters
            .push(ParamItem::new("Result".to_string(), "OK".to_string()));
        Ok(frame)
    } else {
        Err(FrameError::InvalidPacket)
    }
}

pub fn make_packet(frame: SdcpFrame) -> String {
    let mut data: String = String::new();

    write!(data, "Command={};", frame.command).unwrap();
    for parameter in frame.parameters.iter() {
        write!(data, "{}={};", parameter.name, parameter.value).unwrap();
    }

    data
}

pub fn send_packet<D: AsRef<str>, W: IOWrite>(data: D, mut writer: W) -> SendResult {
    let header_bytes = SDCP_PACKET_HEADER.as_bytes();

    let data_bytes = data.as_ref().as_bytes();
    let data_bytes_length = (data_bytes.len() as u32).to_be_bytes();

    writer.write_all(header_bytes)?;
    writer.write_all(&data_bytes_length)?;
    writer.write_all(data_bytes)?;
    Ok(())
}

fn recv_packet<R: IORead>(mut reader: R) -> RecvResult {
    let mut packet_header = [0; SDCP_PACKET_HEADER.len()];
    reader.read_exact(&mut packet_header)?;

    let protocol = str::from_utf8(&packet_header).map_err(|_| RecvError::BadEncoding);
    match protocol {
        Ok(protocol) => {
            if !protocol.eq(SDCP_PACKET_HEADER) {
                return Err(RecvError::InvalidPacket);
            }
        }
        Err(error) => return Err(error),
    }

    let mut data_length = [0; 4];
    reader.read_exact(&mut data_length)?;

    let mut data = vec![0; u32::from_be_bytes(data_length) as _];
    reader.read_exact(&mut data)?;
    String::from_utf8(data).map_err(|_| RecvError::BadEncoding)
}
