use std::collections::HashSet;
use std::ops::Deref;
use std::str::FromStr;

struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn distance_to(&self, other: &Self) -> usize {
        ((self.x as isize - other.x as isize).abs() + (self.y as isize - other.y as isize).abs())
            as usize
    }
}

struct Observation {
    galaxies: Vec<Galaxy>,
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
}

impl Observation {
    fn expand(mut self, n: usize) -> ExpandedUniverse {
        let n = if n == 1 { 1 } else { n - 1 };
        self.galaxies.iter_mut().for_each(|g| {
            g.y += self.empty_rows.iter().filter(|r| **r < g.y).count() * n;
            g.x += self.empty_cols.iter().filter(|c| **c < g.x).count() * n;
        });

        ExpandedUniverse { inner: self }
    }
}

struct ExpandedUniverse {
    inner: Observation,
}

impl Deref for ExpandedUniverse {
    type Target = Observation;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ExpandedUniverse {
    fn calculate_distances_sum(&self) -> usize {
        self.galaxies
            .iter()
            .map(|g| {
                self.galaxies
                    .iter()
                    .map(|o| g.distance_to(o))
                    .sum::<usize>()
            })
            .sum::<usize>()
            / 2
    }
}

impl FromStr for Observation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxies = Vec::new();

        let max_x = s.lines().next().unwrap().len();
        let max_y = s.lines().count();

        let mut empty_rows = HashSet::from_iter(0..max_y);
        let mut empty_cols = HashSet::from_iter(0..max_x);

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        galaxies.push(Galaxy::new(x, y));
                        empty_rows.remove(&y);
                        empty_cols.remove(&x);
                    }
                    '.' => (),
                    _ => panic!("invalid input"),
                }
            }
        }

        Ok(Self {
            galaxies,
            empty_rows,
            empty_cols,
        })
    }
}

fn part1(input: &str) -> usize {
    let observation = Observation::from_str(input).unwrap();
    observation.expand(1).calculate_distances_sum()
}

fn part2(input: &str) -> usize {
    let observation = Observation::from_str(input).unwrap();
    observation.expand(1_000_000).calculate_distances_sum()
}

pub fn day() {
    let input = include_str!("../input/day11.txt");
    print!("day 11\t");
    print!("part 1: {}\t", part1(input));
    print!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    #[test]
    fn test_part1() {
        let map = Observation::from_str(INPUT).unwrap();
        assert_eq!(map.galaxies.len(), 9);
        assert_eq!(map.empty_rows.len(), 2);
        assert_eq!(map.empty_cols.len(), 3);
        let map = map.expand(1);
        assert_eq!(map.galaxies[4].distance_to(&map.galaxies[8]), 9);
        assert_eq!(map.galaxies[0].distance_to(&map.galaxies[6]), 15);
        assert_eq!(map.galaxies[2].distance_to(&map.galaxies[5]), 17);
        assert_eq!(map.galaxies[7].distance_to(&map.galaxies[8]), 5);
        assert_eq!(part1(INPUT), 374);
    }

    #[test]
    fn test_part2() {
        let map = Observation::from_str(INPUT).unwrap();
        let map = map.expand(10);
        assert_eq!(map.calculate_distances_sum(), 1030);
        let map = Observation::from_str(INPUT).unwrap();
        let map = map.expand(100);
        assert_eq!(map.calculate_distances_sum(), 8410);
    }
}
