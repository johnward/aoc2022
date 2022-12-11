use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let mut manager = PacketManager::new();
    manager.read_input("input.txt".into());

    let (found, index) = manager.process();

    println!("Index found ({}): {}", found, index);

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

    pub fn process(&self) -> (bool, usize) {

        if self.packets.len() >= 4 {
            for (i, c) in self.packets.chars().enumerate() {
                if self.unique_chars(&self.packets[i..i+4]) {
                    return (true, i + 4);
                }

                if i == self.packets.len() - 4 {
                    break;
                }
            }
        }   

        return (false, 0);

    }

    fn unique_chars(&self, marker_str: &str) -> bool {

        //for c in marker_str.chars()

        if marker_str.eq("abcd") {
            println!("abcd");
        }

        println!("Marker Str: {}", marker_str);
        
        for (i,m) in marker_str.chars().enumerate() {
            for (j, n) in marker_str.chars().enumerate() {

                if i == j {
                    continue;
                }

                if n.eq_ignore_ascii_case(&m) {
                    return false
                }

            }
        }

        true
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
