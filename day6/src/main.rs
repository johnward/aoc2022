use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let mut manager = PacketManager::new();
    manager.read_input("input.txt".into());

    Ok(())
}

struct PacketManager {
    packets: String,
    marker: Vec<String>
}

impl PacketManager {
    pub fn new() -> PacketManager {
        PacketManager {
            packets: String::new(),
            marker: Vec::new(),
        }
    }

    fn read_input(&mut self, file_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        self.packets = std::fs::read_to_string(file_path)?;

        Ok(())
    }

    pub fn process(&self) {

    }
}
