mod part1;
mod part2;
mod part5;

const PROJECT: u16 = 5;

fn main() {
    if PROJECT == 1  {
        part1::part1();
    } else if PROJECT == 2 {
        part2::part2();
    } else if PROJECT == 5 {
        part5::part5();
    }
}