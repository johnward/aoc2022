use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let mut mgr = DirManager::new();
    let _result = mgr.read_input("input.txt".into());

    Ok(())
}

struct DirManager {
    content: String,
}

impl DirManager {
    pub fn new() -> DirManager {
        DirManager {
            content: String::new(),
        }
    }

    pub fn read_input(&mut self, file_path: PathBuf) -> Result <(), Box<dyn std::error::Error>> {
        self.content = std::fs::read_to_string(file_path)?;

        Ok(())
    }
}