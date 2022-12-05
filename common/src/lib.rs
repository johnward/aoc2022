use std::fs;
use std::io;
use std::path::Path;
use std::io::{BufRead};
use std::fs::File;

/// Read a file
pub fn read_file<T: AsRef<Path>>(a_path: T) -> io::Result<String> {
    fs::read_to_string(a_path)
}

/// Write a file
pub fn write_file<P, C>(a_path: P, data: C) -> io::Result<()> 
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    fs::write(a_path, data)
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = "Hello!!!";
        let result = write_file("afile.txt", &data.as_bytes()).unwrap();
        assert_eq!(result, ());
    }

    #[test]
    fn read_data() {
        let data = "Hello!!!";
        let result = write_file("newfile.txt", &data.as_bytes());

        let result2 = read_file("newfile.txt").unwrap();

        assert_eq!(result2, "Hello!!!");


    }
}

pub fn alphabet_position(text: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let s: String = text.chars().into_iter()
                                .filter(|&c| c.is_alphabetic())
                                .map(|c| c.to_ascii_uppercase())
                                .map(|c| c as u8)
                                .map(|c| (c - 64u8).to_string())
                                .collect();

    Ok(s.parse::<i32>().unwrap())
}

pub fn range_overlaps(vec1: Vec<i32>, vec2: Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    
    let mut is_inner = 0;
    
    if (vec1[0] >= vec2[0] && vec1[0] <= vec2[1]) || 
        (vec1[1] <= vec2[1] && vec1[1] >= vec2[0]) {
        is_inner = 1;
    }

    if (vec2[0] >= vec1[0] && vec2[0] <= vec1[1]) || 
        (vec2[1] <= vec1[1] && vec2[1] >= vec1[0]) {
        is_inner = 1;
    }

    Ok(is_inner)
}


pub fn range_is_inside(vec1: Vec<i32>, vec2: Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    
    let mut is_inner = 0;

    if vec1.len() == 2 && vec2.len() == 2 {
    
        if (vec1[0] >= vec2[0] && vec1[1] <= vec2[1]) || (vec2[0] >= vec1[0] && vec2[1] <= vec1[1]) {
            is_inner = 1;
        }
    }

    Ok(is_inner)
}
