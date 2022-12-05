#![feature(iter_next_chunk)]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Day4");

    part1()?;

    part2()?;

    Ok(())
}

fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("input.txt")?;

    let total: i32 = content.lines().map(|line| {

        let mut is_inner = 0;
        let elfs_vec: Vec<&str> = line.split(",").collect();

        let elf1: Vec<i32> = elfs_vec[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect();
        let elf2: Vec<i32> = elfs_vec[1].split("-").map(|x| x.parse::<i32>().unwrap()).collect();

        if (elf1[0] >= elf2[0] && elf1[1] <= elf2[1]) || (elf2[0] >= elf1[0] && elf2[1] <= elf1[1]) {
            is_inner = 1;
        }
    
        is_inner

    }).sum();

    println!("Total 1: {}", total);
    
    Ok(())
}

fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("input.txt")?;

    let total: i32 = content.lines().map(|line| {

        let mut is_inner = 0;
        let elfs_vec: Vec<&str> = line.split(",").collect();

        let elf1: Vec<i32> = elfs_vec[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect();
        let elf2: Vec<i32> = elfs_vec[1].split("-").map(|x| x.parse::<i32>().unwrap()).collect();


        if (elf1[0] >= elf2[0] && elf1[0] <= elf2[1]) || 
        (elf1[1] <= elf2[1] && elf1[1] >= elf2[0]) {
            is_inner = 1
        }

        if (elf2[0] >= elf1[0] && elf2[0] <= elf1[1]) || 
        (elf2[1] <= elf1[1] && elf2[1] >= elf1[0]) {
            is_inner = 1
        }

        is_inner

    }).sum();

    println!("Total 2: {}", total);
    
    Ok(())
}
