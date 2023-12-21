use std::arch::x86_64::_mm256_div_pd;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(PartialEq)]
enum Field {
    GardenPlot,
    Rock,
}

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
    fn calculate_possible_position(&self, mut num: usize, infinite_map: bool) -> Vec<usize> {
        let mut num = num as isize;
        let is_even = (num % 2 == 0).then(|| 0).or_else(|| Some(1)).unwrap();
        num -= 1;

        let mut found_positions: HashMap<Point, usize> = HashMap::new();
        let mut stack_positions = vec![self.starting_position.clone()];

        while num >= 0 {
            let mut current_stack = stack_positions;
            stack_positions = Vec::new();

            while let Some(pos) = current_stack.pop() {
                let possible_positions = DIRECTIONS.iter().map(|d| pos.clone().add(&d.to_point())).collect::<Vec<_>>();
                for mut pos in possible_positions {
                    if !infinite_map {
                        if pos.x < 0 || pos.y < 0 || pos.x >= self.fields[0].len() as isize || pos.y >= self.fields.len() as isize || self.fields[pos.y as usize][pos.x as usize] == Field::Rock || found_positions.contains_key(&pos) {
                            continue;
                        }
                    } else {
                        if pos.x < 0 {
                            pos.x = self.fields[0].len() as isize - 1;
                        }
                        if pos.y < 0 {
                            pos.y = self.fields.len() as isize - 1;
                        }
                        if pos.y >= self.fields.len() as isize {
                            pos.y = 0;
                        }
                        if pos.x >= self.fields[0].len() as isize {
                            pos.x = 0;
                        }
                        if self.fields[pos.y as usize][pos.x as usize] == Field::Rock && found_positions.contains_key(&pos) {
                            continue;
                        }
                    }
                    if num % 2 == is_even {
                        *found_positions.entry(pos.clone()).or_insert(0) = (num / 2) as usize;
                    }
                    stack_positions.push(pos);
                }
            }
            num -= 1;
        }

        let vec = found_positions.into_values().collect();
        vec
    }
}

fn part1(input: &str, n: usize) -> usize {
    let grid = Grid::from_str(input).unwrap();
    grid.calculate_possible_position(n, false).len()
}

fn part2(input: &str, n: usize) -> usize {
    let grid = Grid::from_str(input).unwrap();
    grid.calculate_possible_position(n, true).iter().sum()
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

    #[test]
    fn test_part2() {
        let grid = Grid::from_str(INPUT).unwrap();
        assert_eq!(part2(INPUT, 6), 16);
        assert_eq!(part2(INPUT, 10), 50);
        assert_eq!(part2(INPUT, 50), 1594);
        assert_eq!(part2(INPUT, 100), 6536);
        assert_eq!(part2(INPUT, 500), 167004);
        assert_eq!(part2(INPUT, 1000), 668697);
        assert_eq!(part2(INPUT, 5000), 16733044);
    }
}
