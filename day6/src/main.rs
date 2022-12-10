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
    marker_check: Vec<char>
}

impl PacketManager {
    pub fn new() -> PacketManager {
        PacketManager {
            packets: String::new(),
            marker_check: Vec::new(),
        }
    }

    fn read_input(&mut self, file_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        self.packets = std::fs::read_to_string(file_path)?;

        Ok(())
    }

    pub fn process(&self) {

    }

    fn check_unique(&self) -> Result<bool, Box<dyn std::error::Error>> {

        for m in &self.marker_check {
            for n in &self.marker_check {

                if n == m {
                    return Ok(false)
                }

            }
        }

        Ok(true)
    }

// in C++
// bool uniqueCharacters(string str)
// {
 
//     // If at any time we encounter 2
//     // same characters, return false
//     for (int i = 0; i < str.length() - 1; i++) {
//         for (int j = i + 1; j < str.length(); j++) {
//             if (str[i] == str[j]) {
//                 return false;
//             }
//         }
//     }
 
    // If no duplicate characters encountered,
    // return true
//     return true;
// }

    pub fn add_marker(&mut self, marker_char: char) -> Result<(), Box<dyn std::error::Error>> {

        self.marker_check.push(marker_char);

        Ok(())
    }
}
