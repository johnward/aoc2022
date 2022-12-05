#![feature(iter_next_chunk)]

use common::{range_is_inside, range_overlaps};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Day4");

    part1()?;

    part2()?;

    Ok(())
}

fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("input.txt")?;

    let total: i32 = content.lines().map(|line| {

        let elfs_vec: Vec<&str> = line.split(",").collect();

        let elf1: Vec<i32> = elfs_vec[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect();
        let elf2: Vec<i32> = elfs_vec[1].split("-").map(|x| x.parse::<i32>().unwrap()).collect();

        let is_inner = range_is_inside(elf1, elf2).unwrap();

        is_inner

    }).sum();

    println!("Total 1: {}", total);
    
    Ok(())
}

fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("input.txt")?;

    let total: i32 = content.lines().map(|line| {

        let elfs_vec: Vec<&str> = line.split(",").collect();

        let elf1: Vec<i32> = elfs_vec[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect();
        let elf2: Vec<i32> = elfs_vec[1].split("-").map(|x| x.parse::<i32>().unwrap()).collect();

        let is_inner = range_overlaps(elf1, elf2).unwrap();

        is_inner

    }).sum();

    println!("Total 2: {}", total);
    
    Ok(())
}
