use std::fmt::Write;
use std::net::SocketAddr;
use std::str;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

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
    pub async fn bind(&self, handler: SdcpRequestHandler) {
        let address = self.address;
        tokio::spawn(async move {
            let listener = match TcpListener::bind(address).await {
                Ok(listener) => listener,
                Err(error) => panic!("Binding error: {}", error),
            };
            loop {
                println!("In loop");
                match listener.accept().await {
                    Ok((mut stream, address)) => {
                        println!("Source address: {}", address);
                        match recv_packet(&stream).await {
                            Ok(packet) => {
                                match send_packet(
                                    make_packet(handler(make_frame(packet))),
                                    &mut stream,
                                )
                                .await
                                {
                                    Ok(()) => (),
                                    Err(error) => println!("Packet sending error: {}", error),
                                }
                            }
                            Err(error) => println!("Error receiving package: {}", error),
                        }
                    }
                    Err(error) => println!("Error receiving package: {}", error),
                }
            }
        });
    }

    pub async fn request(&self, frame: SdcpFrame, address: SocketAddr) -> RequestResult {
        match TcpStream::connect(address).await {
            Ok(mut stream) => match send_packet(make_packet(frame), &mut stream).await {
                Ok(_) => match recv_packet(&stream).await {
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
                frame.command = pair[1].to_string().to_uppercase();
            } else if "Result".to_uppercase().eq(&pair[0].to_uppercase()) {
                frame.result = pair[1].to_string().to_uppercase();
            } else {
                frame.parameters.push(ParamItem::new(
                    pair[0].to_string().to_uppercase(),
                    pair[1].to_string().to_uppercase(),
                ));
            }
        }
    }
    if !frame.parameters.is_empty() {
        Ok(frame)
    } else {
        Err(FrameError::InvalidPacket)
    }
}

pub fn make_packet(frame: SdcpFrame) -> String {
    let mut data: String = String::new();

    write!(data, "Command={};", frame.command).unwrap();
    write!(data, "Result={};", frame.result).unwrap();
    for parameter in frame.parameters.iter() {
        write!(data, "{}={};", parameter.name, parameter.value).unwrap();
    }

    data
}

pub async fn send_packet<D: AsRef<str>>(data: D, stream: &mut TcpStream) -> SendResult {
    let header_bytes = SDCP_PACKET_HEADER.as_bytes();

    let data_bytes = data.as_ref().as_bytes();
    let data_bytes_length = (data_bytes.len() as u32).to_be_bytes();

    stream.write_all(header_bytes).await?;
    stream.write_all(&data_bytes_length).await?;
    stream.write_all(data_bytes).await?;
    Ok(())
}

async fn recv_packet(stream: &TcpStream) -> RecvResult {
    let mut packet = [0; 4096];
    stream.readable().await?;

    match stream.try_read(&mut packet) {
        Ok(0) => Err(RecvError::InvalidPacket),
        Ok(n) => {
            if n < SDCP_PACKET_HEADER.len() + 4 {
                return Err(RecvError::InvalidPacket);
            }
            let protocol = str::from_utf8(&packet[0..SDCP_PACKET_HEADER.len()])
                .map_err(|_| RecvError::BadEncoding);
            match protocol {
                Ok(protocol) => {
                    if !protocol.eq(SDCP_PACKET_HEADER) {
                        return Err(RecvError::InvalidPacket);
                    }
                }
                Err(error) => return Err(error),
            }
            let data_length = u32::from_be_bytes([
                packet[SDCP_PACKET_HEADER.len()],
                packet[SDCP_PACKET_HEADER.len() + 1],
                packet[SDCP_PACKET_HEADER.len() + 2],
                packet[SDCP_PACKET_HEADER.len() + 3],
            ]);
            if n < SDCP_PACKET_HEADER.len() + 4 + (data_length as usize) {
                return Err(RecvError::InvalidPacket);
            }
            let data = packet[SDCP_PACKET_HEADER.len() + 4
                ..SDCP_PACKET_HEADER.len() + 4 + (data_length as usize)]
                .to_vec();
            String::from_utf8(data).map_err(|_| RecvError::BadEncoding)
        }
        Err(_) => Err(RecvError::InvalidPacket),
    }
}
