#[path="common.rs"]
mod common;

fn process(s: &String) -> i32 {
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
    common::common::read_file("input1.txt", &process);
}
