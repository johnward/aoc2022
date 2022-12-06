fn main() {
    println!("Hello, world!");
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
};

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
}

fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("input.txt")?;

    // let vec1 = Vec!["H","T","Z","D"];
    // let vec2 = Vec!["Q","R","W","T","G","C","S"];
    // let vec3 = Vec!["P","B","F","Q","N","R","C","H"];
    // let vec4 = Vec!["L","C","N","F","H","Z"];
    // let vec5 = Vec!["G","L","F","Q","S"];
    // let vec6 = Vec!["V","P","W","Z","B","R","C","S"];
    // let vec7 = Vec!["Z","F","J"];
    // let vec8 = Vec!["D","L","V","Z","R","H","Q"];
    // let vec9 = Vec!["B","H","G","N","F","Z","L","D"];

    // let total: i32 = content.lines().map(|line| {

    // });
    Ok(())
}


//fn move(start: i32, end: i32, vec)
