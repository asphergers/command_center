use std::{fs, time::Duration};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
mod items;
use items::*;

fn main() {
    let handler = build_handler();
    
    handler.start();
}

fn build_handler() -> CommandHandler {
    let config = fs::read_to_string("./config.json").unwrap();
    let commands: Commands = serde_json::from_str(&config).unwrap();
    let mut c_map: HashMap<i32, String> = HashMap::new();

    for c in commands.binds {
        c_map.insert(c.id, c.script_location);
    }

    let port = serialport::new("/dev/ttyACM1", 9600)
                .timeout(Duration::from_millis(10000))
                .open()
                .expect("unable to open port");

    return CommandHandler { 
        command_map: c_map,
        serial_port: port
    }
}

impl CommandHandler {
    pub fn execute_command(&self, id: i32) -> Result<(), &str> {
        let file = match self.command_map.get(&id) {
            Some(x) => x,
            None => return Err("n")
        };

        let contents =  match fs::read_to_string(file) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("ERROR OPENING SCRIPT: {}", e);
                return Ok(())
            }
        };

        match run_script::spawn_script!(&contents) {
            Ok(_) => (),
            Err(e) => { 
                eprintln!("ERROR EXECUTING SCRIPT: {}", e);
                return Ok(())
            }
        }

        Ok(())
    }

    pub fn start(&self) {
        let mut reader = BufReader::new(self.serial_port.try_clone().unwrap());

        loop {
            let mut bytes: Vec<u8> = vec![8; 0];
            reader.read_until(255, &mut bytes).unwrap();

            match self.execute_command(bytes[0] as i32) {
                Ok(_) => (),
                Err(_) => eprintln!("op code not assigned to {}", bytes[0])
            }
        }
    }
}
