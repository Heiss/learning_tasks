use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    let t = Instant::now();

    let mut vec = vec![
        day1::day,
        day2::day,
        day3::day,
        day4::day,
        day5::day,
        day6::day,
        day7::day,
    ];
    if let Ok(v) = args[1].parse::<usize>() {
        vec = vec![vec[v - 1]];
    }
    let vec = vec;

    std::thread::scope(|s| {
        let ts: Vec<_> = vec
            .iter()
            .map(|f| {
                s.spawn(|| {
                    let t = Instant::now();
                    let res = f();
                    (t.elapsed(), res)
                })
            })
            .collect();
        ts.into_iter().for_each(|h| {
            let (t, r) = h.join().expect("Thread");
            println!("{}\tTime: {:?}", r, t);
        });
    });
    println!(
        "-----------------------------------------------\nTotal time: {:?}",
        t.elapsed()
    );
}
