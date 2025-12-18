use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct _Ray {
    x: usize,
    y: usize,
}

impl _Ray {
    fn _next_position(&self) -> _Ray {
        _Ray {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn _split(&self) -> (_Ray, _Ray) {
        (
            _Ray {
                x: self.x - 1,
                y: self.y,
            },
            _Ray {
                x: self.x + 1,
                y: self.y,
            },
        )
    }
}

enum Tile {
    Empty,
    Split,
    Ray,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Split => write!(f, "^"),
            Tile::Ray => write!(f, "|"),
        }
    }
}

struct Manifold {
    map: Vec<Vec<Tile>>,
    rays: Vec<(usize, usize)>,
    start: (usize, usize),
    counter_splits: usize,
    timelines: Vec<Vec<usize>>,
}

impl Display for Manifold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if (x, y) == self.start {
                    write!(f, "S")?;
                } else {
                    write!(f, "{}", tile)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Manifold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Vec::new();
        let mut start = None;
        for (y, l) in s.lines().enumerate() {
            let mut map = Vec::new();

            for (x, c) in l.chars().enumerate() {
                match c {
                    '.' => map.push(Tile::Empty),
                    '|' => {
                        map.push(Tile::Ray);
                    }
                    '^' => map.push(Tile::Split),
                    'S' => {
                        map.push(Tile::Ray);
                        start = Some((x, y));
                    }
                    _ => panic!("Invalid character: {}", c),
                }
            }

            result.push(map);
        }

        let start = start.unwrap();
        let mut timelines = vec![vec![0; result[0].len()]; result.len()];
        timelines[start.1][start.0] = 1;
        let mani = Manifold {
            map: result,
            rays: vec![start],
            start,
            counter_splits: 0,
            timelines,
        };
        Ok(mani)
    }
}

impl Manifold {
    fn next(&mut self) {
        let mut new_rays = Vec::new();
        while let Some(ray) = self.rays.pop() {
            let next_ray = (ray.0, ray.1 + 1);
            if next_ray.1 >= self.map.len() || next_ray.0 >= self.map[0].len() {
                continue;
            }
            match self.map[next_ray.1][next_ray.0] {
                Tile::Empty => {
                    new_rays.push(next_ray);
                }
                Tile::Split => {
                    let rs = vec![(next_ray.0 - 1, next_ray.1), (next_ray.0 + 1, next_ray.1)];
                    for r in &rs {
                        self.timelines[r.1][r.0] += self.timelines[ray.1][ray.0];
                    }
                    new_rays.extend(rs);
                    self.counter_splits += 1;
                }
                Tile::Ray => {}
            }
            self.timelines[next_ray.1][next_ray.0] += self.timelines[ray.1][ray.0];
            for r in &new_rays {
                self.map[r.1][r.0] = Tile::Ray;
            }
        }
        new_rays.sort();
        new_rays.dedup();

        self.rays = new_rays;
    }

    fn calculate_splits(&mut self) -> usize {
        self.calculate();
        self.counter_splits
    }

    fn calculate(&mut self) {
        while self.rays.len() > 0 {
            self.next();
        }
    }

    fn calculate_timelines(&mut self) -> usize {
        self.calculate();
        self.timelines.last().unwrap().iter().sum()
    }
}

fn part1(input: &str) -> usize {
    Manifold::from_str(input).unwrap().calculate_splits()
}

fn part2(input: &str) -> usize {
    Manifold::from_str(input).unwrap().calculate_timelines()
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode25/day7.txt");
    format!("Day 7\tPart 1: {}\t Part 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn day1() {
        assert_eq!(part1(INPUT), 21);
    }
    #[test]
    fn day2() {
        assert_eq!(part2(INPUT), 40);
    }
}
