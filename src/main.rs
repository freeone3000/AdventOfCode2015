use std::fs::File;
use std::io::Read;
use std::path::Path;

fn process(s: String) -> i32 {
    let mut floor = 0;
    for c in s.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("unexpected input: {}", c)
        }
    }
    return floor;
}

fn main() {
    let path = Path::new("input1.txt");
    let mut file = match File::open(&path) {
        Err(why) => panic!("Error opening file: {}", why),
        Ok(file) => file
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Error reading file: {}", why),
        Ok(_) => print!("{}", process(s))
    };
}
