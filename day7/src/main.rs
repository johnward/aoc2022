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

        let output: usize = self.content.split(" ").map(|x| {

            match output[0] {
                $ => {

                },
                _ => println!("Error"),
            }
        });

        Ok(())
    }
}

struct Directory {
    name: String,
    files: Vec<File>,
    subdirectories: Vec<Directory>,
    totalsize: usize,
}

impl Directory {
    pub fn new() -> Directory {
        Directory {
            name: String::new(),
            files: Vec::new(),
            subdirectories: Vec::new(),
            totalsize: 0
        }
    }

    pub fn get_total_size(&self) -> int{
        let totalSize = 0;
        for (const auto& file : self.files) {
          totalSize += file.size;
        }
        for (const auto& subdirectory : subdirectories) {
          totalSize += subdirectory.getTotalSize();
        }
        return totalSize;
      }
}

struct File {
    name: String,
    size: usize,
}

impl File {
    pub fn new() -> File {
        File { name: String::new(), size: 0 }
    }
}