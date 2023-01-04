mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

pub fn run() {
    let input = include_str!("./day1.txt");
    day1::day1(input);
    let input = include_str!("./day2.txt");
    day2::day2(input);
    let input = include_str!("./day3.txt");
    day3::day3(input);
    let input = include_str!("./day4.txt");
    day4::day4(input);
    let input = include_str!("./day5.txt");
    day5::day5(input);
    let input = include_str!("./day6.txt");
    day6::day6(input);
    let input = include_str!("./day7.txt");
    day7::day7(input);
    let input = include_str!("./day8.txt");
    day8::day8(input);
    let input = include_str!("./day10.txt");
    day10::day10(input);
    let input = include_str!("./day11.txt");
    day11::day11(input);
    let input = include_str!("./day12.txt");
    day12::day12(input);
}
