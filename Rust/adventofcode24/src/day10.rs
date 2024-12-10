use std::collections::HashMap;
use std::str::FromStr;

struct Map {
    grid: Vec<Vec<usize>>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            grid: s
                .lines()
                .map(|v| {
                    v.chars()
                        .map(|c| c.to_digit(10).unwrap_or_else(|| 100) as usize)
                        .collect()
                })
                .collect(),
        })
    }
}

impl Map {
    fn find_entries(&self) -> HashMap<usize, Vec<Point>> {
        let mut res: HashMap<usize, Vec<Point>> = HashMap::new();
        for (y, l) in self.grid.iter().enumerate() {
            for (x, num) in l.iter().enumerate() {
                res.entry(*num).or_default().push((x, y));
            }
        }
        res
    }
}

type Point = (usize, usize);
fn distance(a: Point, b: Point) -> f64 {
    println!("a {:?}, b {:?}", a, b);
    (((b.0.abs_diff(a.0)).pow(2) + (b.1.abs_diff(a.1)).pow(2)) as f64).sqrt()
}

fn manhattan_distance(a: Point, b: Point) -> usize {
    distance(a, b).ceil() as usize
}

fn part1(input: &str) -> usize {
    let entries = Map::from_str(input).unwrap().find_entries();
    for i in 0..=9 {
        todo!("Which approach could be the best? A*?")
    }
    0
}

fn part2(input: &str) -> usize {
    0
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode24/day10.txt");
    format!(
        "Day 10\tPart 1: {}\t Part 2: {}",
        part1(input),
        part2(input)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
    #[test]
    fn day1() {
        assert_eq!(
            part1(
                r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#
            ),
            4
        );
        assert_eq!(
            part1(
                r#"10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01"#
            ),
            2
        );
        assert_eq!(
            part1(
                r#"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"#
            ),
            2
        );
        assert_eq!(part1(INPUT), 36);
    }
    #[test]
    fn day2() {
        assert_eq!(part2(INPUT), 0);
    }
}
