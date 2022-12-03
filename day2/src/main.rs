use common::{read_lines};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    day2_part1()?;

    day2_part2()?;

    Ok(())
}

fn day2_part2() -> Result<(), Box<dyn std::error::Error>> {

    let content = std::fs::read_to_string("input.txt")?;

    let mut total = 0; 

    for line in content.lines() {
        
        //println!("line: {}", line);
        
        let game: Vec<&str> = line.split(" ").collect();
        
        if game.len() == 2 {
            let opponent = game[0];
            let you = game[1];

            match opponent {
                "A" => {
                    if you == "X" {
                        // Lose
                        total += 3;
                    } else if you == "Y" {
                        // Draw
                        total += 4;
                    } else if you == "Z" {
                        // Win
                        total += 8;
                    }
                },
                "B" => {
                    if you == "X" {
                        // Lose
                        total += 1;
                    } else if you == "Y" {
                        // Draw
                        total += 5;
                    } else if you == "Z" {
                        // Win
                        total += 9;
                    }
                },
                "C" => {
                    if you == "X" {
                        // Lose
                        total += 2;
                    } else if you == "Y" {
                        // Draw
                        total += 6;
                    } else if you == "Z" {
                        // Win
                        total += 7;
                    }
                },
                _ => println!("Error"),
            }
        }
    
    }

    println!("Total (Part 2): {}", total);

    Ok(())

}


fn day2_part1() -> Result<(), Box<dyn std::error::Error>> {

    let content = std::fs::read_to_string("input.txt")?;

    let mut total = 0; 

    for line in content.lines() {
        
        //println!("line: {}", line);
        
        let game: Vec<&str> = line.split(" ").collect();
        
        if game.len() == 2 {
            let opponent = game[0];
            let you = game[1];

            match opponent {
                "A" => {
                    if you == "X" {
                        total += 4;
                    } else if you == "Y" {
                        total += 8;
                    } else if you == "Z" {
                        total += 3;
                    }
                },
                "B" => {
                    if you == "X" {
                        total += 1;
                    } else if you == "Y" {
                        total += 5;
                    } else if you == "Z" {
                        total += 9
                    }
                },
                "C" => {
                    if you == "X" {
                        total += 7;
                    } else if you == "Y" {
                        total +=2;
                    } else if you == "Z" {
                        total += 6;
                    }
                },
                _ => println!("Error"),
            }
        }
    
    }

    println!("Total (Part 1): {}", total);

    Ok(())

}
