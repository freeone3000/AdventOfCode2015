use std::cmp::min;
use std::str::FromStr;

#[path="common.rs"]
mod common;

struct PresentDimensions {
    l: i32,
    w: i32,
    h: i32
}

impl FromStr for PresentDimensions {
    type Err = std::num::ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        const LENGTH: usize = 3; // number of elements in PresentDimensions
        let mut parts: [i32; LENGTH] = [0, 0, 0];
        let mut part_idx = 0;

        for part in line.trim_end().split('x') {
            parts[part_idx] = i32::from_str(&part)?;
            part_idx += 1;
            if part_idx >= parts.len() {
                break;
            }
        }

        return Ok(PresentDimensions { l: parts[0], w: parts[1], h: parts[2] })
    }
}

fn calculate_surface_area(dims: &PresentDimensions) -> i32 {
    let size1 = dims.l*dims.w;
    let size2 = dims.w*dims.h;
    let size3 = dims.h*dims.l;
    let slack = min(size1, min(size2, size3));
    return size1 + size1 + size2 + size2 + size3 + size3 + slack;
}

/*
The ribbon required to wrap a present is the shortest distance around its sides,
or the smallest perimeter of any one face. Each present also requires a bow made
out of ribbon as well; the feet of ribbon required for the perfect bow is equal
to the cubic feet of volume of the present. Don't ask how they tie the bow, though;
they'll never tell.
 */
fn calculate_ribbon(dims: &PresentDimensions) -> i32 {
    let perim1 = dims.l + dims.l + dims.w + dims.w;
    let perim2 = dims.w + dims.w + dims.h + dims.h;
    let perim3 = dims.h + dims.h + dims.l + dims.l;
    let bow = dims.l * dims.w * dims.h;
    return min(perim1, min(perim2, perim3)) + bow;
}

fn process_present(line: &String) -> PresentDimensions {
    let dims: PresentDimensions = line.parse().unwrap();
    return dims;
}

pub fn part2() {
    let mut surface_area = 0;
    let mut ribbon = 0;
    for dims in common::common::read_file_linewise("input2.txt", &process_present) {
        surface_area += calculate_surface_area(&dims);
        ribbon += calculate_ribbon(&dims);
    }
    println!("Surface area: {}", surface_area);
    println!("Ribbon: {}", ribbon);
}