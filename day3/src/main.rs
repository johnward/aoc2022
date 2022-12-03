use common::{alphabet_position};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    //part1()?;

    part2()?;

    Ok(())
}

fn part1() -> Result<(), Box<dyn std::error::Error>> {

    let content = std::fs::read_to_string("input.txt")?;

    //let mut matches = Vec::new();
    let mut match_ch = 'a';
    let mut match_ch_vec = Vec::new();
    let mut match_count = 0;
    let mut total = 0;

    for line in content.lines() {

        let line_str = String::from(line);
        let (comp1, comp2) = line_str.split_at(line_str.len()/2);

        for ch1 in comp1.chars() {
            for ch2 in comp2.chars() {
                if ch1 == ch2 {
                    match_ch = ch1;
                    match_ch_vec.push(match_ch);
                    match_count += 1;
                }
            }
        }

        let mut tmp = [0; 4];
        let mut pos = alphabet_position(match_ch.encode_utf8(&mut tmp)).unwrap();

        if match_ch.is_uppercase() {
            pos = pos + 26;
        }

        match_ch_vec.clear();
        println!("pos {}", pos);
        match_count = 0;
        total += pos;
    }

    println!("Total: {}", total);

    Ok(())

}


fn part2() -> Result<(), Box<dyn std::error::Error>> {

    let content = std::fs::read_to_string("input.txt")?;

    let mut elf1= String::from("a");
    let mut elf2= String::from("b");
    let mut elf3= String::from("c");

    let mut count = 0;
    let mut total = 0;

    
    for line in content.lines() {

        match count {
            0 => {
                elf1 = String::from(line);
                println!("elf1: {}", elf1);
            },
            1 => {
                elf2 = String::from(line);
                println!("elf2: {}", elf2);
            },
            2 => {
                elf3 = String::from(line);
                println!("elf3: {}", elf3);
                // do the searching here

                for txt in elf1.chars() {
                    //println!("char: {}", txt);

                    let elf2_find = find_char(txt, &elf2);
                    let elf3_find = find_char(txt, &elf3);

                    if elf2_find != None && elf3_find != None {
                        println!("Badge: {}", txt);

                        let mut tmp = [0; 4];
                        let mut pos = alphabet_position(txt.encode_utf8(&mut tmp)).unwrap();

                        if txt.is_uppercase() {
                            pos = pos + 26;
                        }

                        total += pos;
                        break;
                    }
                }
            },
            _ => println!("Error"),
        }
        if count == 2 {
            count = 0;
        } else {
            count += 1;
        }        
    }
    println!("Total: {}", total);

    Ok(())
}

fn find_char(a_char: char, a_str: &String) -> Option<usize> {
    a_str.find(a_char)
}



