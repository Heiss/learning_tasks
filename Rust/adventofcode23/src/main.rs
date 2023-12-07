mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let vec = vec![
        day1::day,
        day2::day,
        day3::day,
        day4::day,
        day5::day,
        day6::day,
        day7::day,
    ];
    std::thread::scope(|s| {
        vec.iter()
            .map(|f| (Instant::now(), s.spawn(|| f())))
            .for_each(|(t, h)| {
                h.join().unwrap();
                println!("Time: {:?}", t.elapsed());
            });
    });
}
