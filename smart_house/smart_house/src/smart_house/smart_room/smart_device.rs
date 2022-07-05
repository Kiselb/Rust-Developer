use sdcp::results::NetResult;

pub const DEVICE_IDENTITY_MIN_LENGTH: usize = 8;

pub trait SmartDevice {
    fn identity(&self) -> &String;
    fn info(&self) -> String;
    fn getp(&self) -> NetResult;
    fn setp(&self) -> NetResult;
}
