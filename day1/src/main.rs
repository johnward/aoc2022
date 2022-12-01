use common::{read_lines};
fn main() {
    println!("Hello, world!");

    //let result = write_file("a_path.txt", "Hello".as_bytes()).expect("Write Error");

    let mut highest: u32 = 0;
    let mut elf_cal: u32 = 0;

    println!("elf_cal (initial) {}", elf_cal);

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(cal) = line {
                if cal != "" {
                    let number = cal.parse::<u32>().unwrap();
                    //let number:u32 = cal.trim().parse().unwrap();
                    elf_cal += number;
                  
                } else {
                    if elf_cal > highest 
                    {
                        highest = elf_cal;
                    } 

                    elf_cal = 0;
                }

                println!("elf_cal (back to zero): {}", elf_cal);
            }
        }
    }

    println!("Highest Cal: {}",highest);

}
