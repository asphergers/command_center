use std::collections::HashMap;
use serde::{Deserialize};
use serialport::SerialPort;

#[derive(Deserialize)]
pub struct Commands {
    pub binds: Vec<Command>
}

#[derive(Deserialize)]
pub struct Command {
    pub id: i32,
    pub script_location: String
}

pub struct CommandHandler {
    pub command_map: HashMap<i32, String>,
    pub serial_port: Box<dyn SerialPort>
}

