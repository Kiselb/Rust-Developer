use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

use sdcp::results::FrameResult;
use sdcp::{
    ParamItem, SdcpFrame, SdcpHandler, SDCP_COMMANDS, SDCP_FAILED, SDCP_OK, SDCP_PACKET_HEADER,
    SDCP_PARAM_PWRCON, SDCP_PARAM_STATUS,
};

struct ElectricSocket {
    power_consumption: u32,
    status: bool,
}

static mut ES: ElectricSocket = ElectricSocket {
    power_consumption: 0,
    status: false,
};

fn handler(frame: FrameResult) -> SdcpFrame {
    match frame {
        Ok(frame) => {
            println!("Protocol: {}", frame.protocol);
            println!("Commnad: {}", frame.command);
            for item in frame.parameters.iter() {
                println!("Parameter: {}={}", item.name, item.value);
            }
            let command = frame.command.as_str();
            let mut response: SdcpFrame = SdcpFrame {
                protocol: SDCP_PACKET_HEADER.to_string(),
                command: command.to_string(),
                parameters: vec![],
                result: SDCP_OK.to_string(),
            };
            let err_response: SdcpFrame = SdcpFrame {
                protocol: SDCP_PACKET_HEADER.to_string(),
                command: command.to_string(),
                parameters: vec![],
                result: SDCP_FAILED.to_string(),
            };
            match command {
                SDCP_COMMANDS::GETP => {
                    for item in frame.parameters.iter() {
                        let item_name = item.name.as_str();
                        match item_name {
                            SDCP_PARAM_STATUS => unsafe {
                                response.parameters.push(ParamItem::new(
                                    item_name.to_string(),
                                    ES.status.to_string(),
                                ))
                            },
                            SDCP_PARAM_PWRCON => unsafe {
                                response.parameters.push(ParamItem::new(
                                    item_name.to_string(),
                                    ES.power_consumption.to_string(),
                                ))
                            },
                            _ => response.parameters.push(ParamItem::new(
                                item_name.to_string(),
                                String::from("UNKNOWN"),
                            )),
                        }
                    }
                    response
                }
                SDCP_COMMANDS::SETP => {
                    for item in frame.parameters.iter() {
                        let item_name = item.name.as_str();
                        let item_value = item.value.as_str();
                        match item_name {
                            SDCP_PARAM_STATUS => {
                                match bool::from_str(item_value.to_lowercase().as_str()) {
                                    Ok(value) => {
                                        unsafe {
                                            ES.status = value;
                                        }
                                        response.parameters.push(ParamItem::new(
                                            item_name.to_string(),
                                            item_value.to_string(),
                                        ))
                                    }
                                    Err(_) => {
                                        return err_response;
                                    }
                                }
                            }
                            SDCP_PARAM_PWRCON => match u32::from_str(item_value) {
                                Ok(value) => {
                                    unsafe {
                                        ES.power_consumption = value;
                                    }
                                    response.parameters.push(ParamItem::new(
                                        item_name.to_string(),
                                        item_value.to_string(),
                                    ))
                                }
                                Err(_) => {
                                    println!("Value conversation failed");
                                    return err_response;
                                }
                            },
                            _ => response.parameters.push(ParamItem::new(
                                item_name.to_string(),
                                item_value.to_string(),
                            )),
                        }
                    }
                    response
                }
                SDCP_COMMANDS::INFO => response,
                _ => response,
            }
        }
        Err(_) => {
            let err_response: SdcpFrame = SdcpFrame {
                protocol: SDCP_PACKET_HEADER.to_string(),
                command: SDCP_COMMANDS::NONE.to_string(),
                parameters: vec![],
                result: SDCP_FAILED.to_string(),
            };
            err_response
        }
    }
}

fn main() {
    println!("Smart electric socket simulator");

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 55100);
    let sdcp = SdcpHandler::new(address);
    sdcp.bind(handler);

    loop {
        let exit = String::from("exit");
        let mut buffer: String = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        if buffer.trim().eq(&exit) {
            std::process::exit(0);
        }
    }
}
