use common::{alphabet_position};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    part1()?;

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



