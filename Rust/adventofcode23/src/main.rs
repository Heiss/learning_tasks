use std::time::Instant;

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
mod day9;
mod day13;
mod day14;

fn main() {
    let vec = vec![
        day1::day,
        day2::day,
        day3::day,
        day4::day,
        //day5::day,
        day6::day,
        day7::day,
        day8::day,
        day9::day,
        day10::day,
        day11::day,
        //day12::day,
        day13::day,
        day14::day,
    ];
    std::thread::scope(|s| {
        vec.iter()
            .map(|f| (Instant::now(), s.spawn(|| f())))
            .for_each(|(t, h)| {
                h.join().expect("Thread");
                println!("Time: {:?}", t.elapsed());
            });
    });
}
