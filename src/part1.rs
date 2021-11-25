use std::fs::File;
use std::io::Read;
use std::path::Path;

fn process(s: String) -> i32 {
    let mut floor = 0;
    let mut position = 1; // by puzzle spec
    let mut printed = false;

    for c in s.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("unexpected input: {}", c)
        };
        if (floor < 0) && !printed {
            println!("First position: {}", position);
            printed = true;
        }
        position += 1;
    }
    return floor;
}

pub fn part1() {
    let path = Path::new("input1.txt");
    let mut file = match File::open(&path) {
        Err(why) => panic!("Error opening file: {}", why),
        Ok(file) => file
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Error reading file: {}", why),
        Ok(_) => println!("Ending floor: {}", process(s))
    };
}
