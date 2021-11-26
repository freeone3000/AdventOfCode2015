mod part1;
mod part2;

const PROJECT: u16 = 2;

fn main() {
    if PROJECT == 1  {
        part1::part1();
    } else if PROJECT == 2 {
        part2::part2();
    }
}