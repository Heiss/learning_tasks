use std::str::FromStr;

struct Point {
    x: isize,
    y: isize,
}

struct Config {
    a: Point,
    b: Point,
    target: Point,
}

fn div(a: isize, b: isize) -> (isize, isize) {
    let x = a / b;
    let y = a % b;
    (x, y)
}

impl Config {
    fn calculate_min_token(&self, cost_a: usize, cost_b: usize) -> Option<(usize, Point)> {
        let x = div(
            self.b.y * self.target.x - self.b.x * self.target.y,
            self.b.y * self.a.x - self.b.x * self.a.y,
        );
        let y = div(
            self.a.x * self.target.y - self.a.y * self.target.x,
            self.b.y * self.a.x - self.b.x * self.a.y,
        );

        if x.1 != 0 || y.1 != 0 {
            return None;
        }

        let tokens = x.0.abs() * cost_a as isize + y.0.abs() * cost_b as isize;
        Some((tokens as usize, Point { x: x.0, y: y.0 }))
    }
}

struct Configurations {
    configs: Vec<Config>,
}

impl Configurations {
    fn correct_target_position(&mut self, offset: usize) {
        for c in self.configs.iter_mut() {
            c.target.x += offset as isize;
            c.target.y += offset as isize;
        }
    }
}

impl FromStr for Configurations {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut configs = Vec::new();
        for s in s.split("\n\n") {
            let mut init_configs: Vec<isize> = Vec::with_capacity(6);
            let mut current = String::new();

            for c in s.chars() {
                if c.is_digit(10) {
                    current.push(c);
                } else if !current.is_empty() {
                    init_configs.push(current.parse().unwrap());
                    current = String::new();
                }
            }

            if !current.is_empty() {
                init_configs.push(current.parse().unwrap());
            }

            configs.push(Config {
                a: Point {
                    x: init_configs.remove(0),
                    y: init_configs.remove(0),
                },
                b: Point {
                    x: init_configs.remove(0),
                    y: init_configs.remove(0),
                },
                target: Point {
                    x: init_configs.remove(0),
                    y: init_configs.remove(0),
                },
            });
        }

        Ok(Self { configs })
    }
}

fn part1(input: &str) -> usize {
    Configurations::from_str(input)
        .unwrap()
        .configs
        .iter()
        .filter_map(|c| c.calculate_min_token(3, 1))
        .filter_map(|(res, Point { x, y })| {
            if x <= 100 && y <= 100 && x >= 0 && y >= 0 {
                Some(res)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut config = Configurations::from_str(input).unwrap();
    config.correct_target_position(10000000000000);
    config
        .configs
        .iter()
        .filter_map(|c| c.calculate_min_token(3, 1))
        .filter_map(|(res, Point { x, y })| {
            if x >= 0 && y >= 0 {
                Some(res)
            } else {
                None
            }
        })
        .sum()
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode24/day13.txt");
    format!(
        "Day 13\tPart 1: {}\t Part 2: {}",
        part1(input),
        part2(input)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;
    #[test]
    fn day1() {
        assert_eq!(part1(INPUT), 480);
    }
    #[test]
    fn day2() {
        //        assert_eq!(part2(INPUT), 1206);
    }
}
