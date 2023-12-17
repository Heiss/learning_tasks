use std::time::Instant;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
        day15::day,
        //day16::day,
    ];
    std::thread::scope(|s| {
        let ts: Vec<_> = vec.iter().map(|f| s.spawn(|| f())).collect();
        ts.into_iter().for_each(|h| {
            let t = Instant::now();
            let r = h.join().expect("Thread");
            println!("{}\tTime: {:?}", r, t.elapsed());
        });
    });
}
