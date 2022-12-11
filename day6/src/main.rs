use std::path::PathBuf;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let mut manager = PacketManager::new();
    let _result = manager.read_input("input.txt".into());

    let (found, index) = manager.process(4);

    println!("Index found ({}): {}", found, index);

    let (found2, index2) = manager.process(14);

    println!("Index found ({}): {}", found2, index2);

    Ok(())
}

struct PacketManager {
    packets: String,
}

impl PacketManager {
    pub fn new() -> PacketManager {
        PacketManager {
            packets: String::new(),
        }
    }

    fn read_input(&mut self, file_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        self.packets = std::fs::read_to_string(file_path)?;

        Ok(())
    }

    pub fn process(&self, message_length: usize) -> (bool, usize) {

        if self.packets.len() >= message_length {
            for (i, _c) in self.packets.chars().enumerate() {
                if self.unique_chars(&self.packets[i..i+message_length]) {
                    return (true, i + message_length);
                }

                if i == self.packets.len() - message_length {
                    break;
                }
            }
        }   

        return (false, 0);

    }

    fn unique_chars(&self, marker_str: &str) -> bool {

        //println!("Marker Str: {}", marker_str);
        
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

}
