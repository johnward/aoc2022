fn main() -> Result<(), Box< dyn std::error::Error>> {
    println!("Hello, world!");

    part2()
}


struct CrateStacks {
    vec1: Vec<char>,
    vec2: Vec<char>,
    vec3: Vec<char>,
    vec4: Vec<char>,
    vec5: Vec<char>,
    vec6: Vec<char>,
    vec7: Vec<char>,
    vec8: Vec<char>,
    vec9 : Vec<char>,
}

impl CrateStacks {
    pub fn new() -> CrateStacks {
        CrateStacks { vec1: vec!['H','T','Z','D'],
        vec2: vec!['Q','R','W','T','G','C','S'],
        vec3: vec!['P','B','F','Q','N','R','C','H'],
        vec4: vec!['L','C','N','F','H','Z'],
        vec5: vec!['G','L','F','Q','S'],
        vec6: vec!['V','P','W','Z','B','R','C','S'],
        vec7: vec!['Z','F','J'],
        vec8: vec!['D','L','V','Z','R','H','Q'],
        vec9: vec!['B','H','G','N','F','Z','L','D'],}
    }

    pub fn move_single_stack(&mut self, number: i32, from: i32, to: i32) {
        
        for _x in 0..number {
            let value = self.get_vec(from).pop();
            self.get_vec(to).push(value.unwrap());
        }
    }

    pub fn move_multiple_stack(&mut self, number: i32, from: i32, to: i32) {
        let mut temp_vec = Vec::new();

        for _x in 0..number {
            let value = self.get_vec(from).pop();
            temp_vec.push(value.unwrap());
        }

        for _x in 0..number {
            self.get_vec(to).push(temp_vec.pop().unwrap());
        }

    }

    fn get_vec(&mut self, vec_number: i32) -> &mut Vec<char> {
        
        match vec_number {
            1 => return &mut self.vec1,
            2 => return &mut self.vec2,
            3 => return &mut self.vec3,
            4 => return &mut self.vec4,
            5 => return &mut self.vec5,
            6 => return &mut self.vec6,
            7 => return &mut self.vec7,
            8 => return &mut self.vec8,
            9 => return &mut self.vec9,
            _ => return &mut self.vec1,
        }

    }

    pub fn get_top_elements(&mut self) -> String {
        let mut top_string = String::from(self.get_vec(1)[0]);
        top_string.push(*self.get_vec(2).last().unwrap());
        top_string.push(*self.get_vec(3).last().unwrap());
        top_string.push(*self.get_vec(4).last().unwrap());
        top_string.push(*self.get_vec(5).last().unwrap());
        top_string.push(*self.get_vec(6).last().unwrap());
        top_string.push(*self.get_vec(7).last().unwrap());
        top_string.push(*self.get_vec(8).last().unwrap());
        top_string.push(*self.get_vec(9).last().unwrap());

        top_string
    }

}

fn part1() -> Result<(), Box<dyn std::error::Error>> {

    let mut stacks = CrateStacks::new();

    let content = std::fs::read_to_string("input.txt")?;

    let lines_processed: i32 = content.lines().map(|line| {
        let line1 = line.replace("move ", "");
        let line2 = line1.replace("from ", "");
        let line3 = line2.replace("to ", "");

        let mut values = line3.split(" ");
        let number = values.next().unwrap().parse::<i32>().unwrap();
        let from = values.next().unwrap().parse::<i32>().unwrap();
        let to = values.next().unwrap().parse::<i32>().unwrap();

        stacks.move_single_stack(number, from, to);

        1
    }).sum();

    println!("Lines Processed: {}", lines_processed);

    println!("Top elements: {}", stacks.get_top_elements());
    
    Ok(())
}

fn part2() -> Result<(), Box<dyn std::error::Error>> {

    let mut stacks = CrateStacks::new();

    let content = std::fs::read_to_string("input.txt")?;

    let lines_processed: i32 = content.lines().map(|line| {
        let line1 = line.replace("move ", "");
        let line2 = line1.replace("from ", "");
        let line3 = line2.replace("to ", "");

        let mut values = line3.split(" ");
        let number = values.next().unwrap().parse::<i32>().unwrap();
        let from = values.next().unwrap().parse::<i32>().unwrap();
        let to = values.next().unwrap().parse::<i32>().unwrap();

        stacks.move_multiple_stack(number, from, to);

        1
    }).sum();

    println!("Lines Processed: {}", lines_processed);

    println!("Top elements: {}", stacks.get_top_elements());
    
    Ok(())
}


//fn move(start: i32, end: i32, vec)
