use std::fs::File;

mod day1;

pub fn run() {
    let input = include_str!("../../../input/Rust/adventofcode20/day1.txt");
    day1::day1(input);
}
