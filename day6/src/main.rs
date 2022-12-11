use std::path::PathBuf;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let mut manager = PacketManager::new();
    let _result = manager.read_input("input.txt".into());

    let (found, index) = manager.process();

    println!("Index found ({}): {}", found, index);

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

    pub fn process(&self) -> (bool, usize) {

        if self.packets.len() >= 4 {
            for (i, _c) in self.packets.chars().enumerate() {
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
