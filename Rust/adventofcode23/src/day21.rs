// Finished part2 with help of https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
// In the end, i had a missing line in input and did not see the assumptions in the INPUT... The example is not enough to solve this in an appropriate time.

use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

#[derive(PartialEq)]
enum Field {
    GardenPlot,
    Rock,
}

type Steps = usize;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_point(&self) -> Point {
        match self {
            Self::Up => Point::new(0, -1),
            Self::Down => Point::new(0, 1),
            Self::Left => Point::new(-1, 0),
            Self::Right => Point::new(1, 0)
        }
    }
}

struct Grid {
    fields: Vec<Vec<Field>>,
    starting_position: Point,
}

const DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = Vec::new();
        let mut starting_position = None;
        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let field = match c {
                    '.' => Field::GardenPlot,
                    '#' => Field::Rock,
                    'S' => {
                        starting_position = Some(Point::new(x as isize, y as isize));
                        Field::GardenPlot
                    }
                    _ => return Err(())
                };
                row.push(field);
            }
            fields.push(row);
        }
        Ok(Self { fields, starting_position: starting_position.unwrap() })
    }
}

impl Grid {
    fn debug(&self, positions: Vec<Point>) {
        for (y, row) in self.fields.iter().enumerate() {
            for (x, field) in row.iter().enumerate() {
                if positions.contains(&Point::new(x as isize, y as isize)) {
                    print!("O");
                } else {
                    match field {
                        Field::GardenPlot => print!("."),
                        Field::Rock => print!("#"),
                    }
                }
            }
            print!("\n");
        }
        print!("\n");
    }


    fn calculate_possible_position(&self) -> HashMap<Point, Steps> {
        let mut frontier = VecDeque::<(Point, usize)>::new();
        let mut visited = HashMap::new();
        frontier.push_back((self.starting_position.clone(), 0));

        let dim_y = self.fields.len() as isize;
        let dim_x = self.fields[0].len() as isize;

        while let Some((coord, dist)) = frontier.pop_front() {
            if visited.contains_key(&coord) {
                continue;
            }

            visited.insert(coord.clone(), dist);

            for d in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                let next = coord.add(&d.to_point());

                if next.x >= 0 && next.x < dim_x && next.y >= 0 && next.y < dim_y {
                    if !visited.contains_key(&next) && self.fields[next.y as usize][next.x as usize] != Field::Rock {
                        frontier.push_back((next, dist + 1));
                    }
                }
            }
        }

        visited
    }
}

fn part1(input: &str, n: usize) -> usize {
    let grid = Grid::from_str(input).unwrap();
    grid.calculate_possible_position().values().filter(|v| **v <= n && **v % 2 == 0).count()
}

fn part2(input: &str, n: usize) -> usize {
    let grid = Grid::from_str(input).unwrap();

    const STEPS_TO_EDGE: usize = 65;
    let visited = grid.calculate_possible_position();

    let even_corners = visited
        .values()
        .filter(|v| **v % 2 == 0 && **v > STEPS_TO_EDGE)
        .count();
    let odd_corners = visited
        .values()
        .filter(|v| **v % 2 == 1 && **v > STEPS_TO_EDGE)
        .count();

    let even_full = visited.values().filter(|v| **v % 2 == 0).count();
    let odd_full = visited.values().filter(|v| **v % 2 == 1).count();

    let dim_y = grid.fields.len();
    let dim_x = grid.fields[0].len();

    // This is 202300 but im writing it out here to show the process
    let n = (n - (dim_y / 2)) / dim_x;
    assert_eq!(n, 202300);

    let even = n * n;
    let odd = (n + 1) * (n + 1);

    let p2 = odd * odd_full + even * even_full - (n + 1) * odd_corners + n * even_corners;

    p2
}

pub fn day() -> String {
    let input = include_str!("../input/day21.txt");
    format!("Day 21\tPart 1: {}\tPart 2: {}", part1(input, 64), part2(input, 26501365))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

    #[test]
    fn test_part1() {
        let grid = Grid::from_str(INPUT).unwrap();
        assert_eq!(part1(INPUT, 6), 16);
    }
}
