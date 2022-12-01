use common::{read_lines};
fn main() {

    day1_part1();
}


fn day1_part1() {
    let mut highest: Vec<u32> = Vec::new();  
    let mut elf_cal: u32 = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(cal) = line {
                if cal != "" {
                    let number = cal.parse::<u32>().unwrap();
                    //let number:u32 = cal.trim().parse().unwrap();
                    elf_cal += number;
                  
                } else {
                    highest.push(elf_cal);

                    elf_cal = 0;
                }
            }
        }
    }

    highest.sort_by(|a, b| b.cmp(a));

    println!("Highest: {:?}", highest);

    let total = highest[0] + highest[1] + highest[2];

    println!("Top three: {}", total);

}
