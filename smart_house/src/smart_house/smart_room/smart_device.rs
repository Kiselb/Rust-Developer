pub trait SmartDevice {
    fn identity(&self) -> &String;
    fn info(&self) -> String;
}
