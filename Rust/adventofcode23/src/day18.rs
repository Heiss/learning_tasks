use std::fmt::{Debug, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl FromStr for RGB {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use regex::Regex;

        let re = Regex::new(r"#([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})").unwrap();
        let caps = re.captures(s).unwrap();
        let r = u8::from_str_radix(&caps[1], 16).unwrap();
        let g = u8::from_str_radix(&caps[2], 16).unwrap();
        let b = u8::from_str_radix(&caps[3], 16).unwrap();
        Ok(RGB { r, g, b })
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Field {
    DigOut(RGB),
    Dirt,
}


struct Plan {
    dir: Direction,
    steps: usize,
    rgb: RGB,
}

impl Plan {
    fn new(dir: Direction, steps: usize, rgb: RGB) -> Self {
        Plan { dir, steps, rgb }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction {}", s),
        }
    }

    fn to_point(&self) -> Point {
        match self {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }

    fn add(&self, p: Point) -> Point {
        let dir = self.to_point();
        Point::new(p.x + dir.x, p.y + dir.y)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
}

#[derive(PartialEq, Clone)]
struct Grid {
    fields: Vec<Vec<Field>>,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.fields {
            for field in row {
                match field {
                    Field::Dirt => write!(f, ".")?,
                    Field::DigOut(rgb) => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut plans = Vec::new();
        for line in s.lines() {
            let mut plan = line.split_whitespace().collect::<Vec<_>>();
            let dir = Direction::from_str(plan[0]);
            let steps = plan[1].parse().unwrap();
            let rgb = RGB::from_str(plan[2]).unwrap();
            plans.push(Plan::new(dir, steps, rgb));
        }

        let mut pos = Point::new(0, 0);

        let mut possitions = Vec::new();
        for plan in plans {
            for _ in 0..plan.steps {
                pos = plan.dir.add(pos);
                possitions.push(pos.clone());
            }
        }

        let find_min_x = possitions.iter().map(|p| p.x).min().unwrap();
        let find_max_x = possitions.iter().map(|p| p.x).max().unwrap();
        let find_min_y = possitions.iter().map(|p| p.y).min().unwrap();
        let find_max_y = possitions.iter().map(|p| p.y).max().unwrap();

        println!("{} {} {} {}", find_min_x, find_max_x, find_min_y, find_max_y);
        let mut fields = vec![vec![Field::Dirt; (find_max_x - find_min_x) as usize + 1]; (find_max_y - find_min_y) as usize + 1];

        for pos in possitions {
            let x = pos.x - find_min_x;
            let y = pos.y - find_min_y;
            fields[y as usize][x as usize] = Field::DigOut(RGB { r: 0, g: 0, b: 0 });
        }

        Ok(Grid { fields })
    }
}

impl Grid {
    /// Count the number of inner dirt fields.
    /// Here it is done through a pixel by pixel search.
    ///
    /// It goes from left to right, top to bottom.
    /// If it encounter a dig out field, it looksup the field up and down, if there is a digout field, too and save the direction.
    /// It goes to the next field and checks for a digout field. If up and down are digout fields too, it checks if the directions are the same.
    /// If the direction changes, all next dirt fields in row are inside and should be counted.
    fn count_inner_dirt_pixel(&self) -> usize {
        let mut count = 0;
        let mut grid = self.clone();

        for y in 1..self.fields.len() - 1 {
            let mut inner = false;
            let mut dir = None;
            for x in 0..self.fields[y].len() {
                match self.fields[y][x] {
                    Field::Dirt => {
                        if inner {
                            grid.fields[y][x] = Field::DigOut(RGB { r: 0, g: 0, b: 0 });
                        }
                        count += inner as usize;
                        dir = None;
                    }
                    Field::DigOut(_) => {
                        let up = self.fields.get(y - 1).map(|row| row[x]);
                        let down = self.fields.get(y + 1).map(|row| row[x]);

                        match (up, down) {
                            (Some(Field::DigOut(_)), Some(Field::DigOut(_))) => {
                                inner = !inner
                            }
                            (Some(Field::DigOut(_)), _) => {
                                if dir == Some(Direction::Down) {
                                    inner = !inner;
                                } else {
                                    dir = Some(Direction::Up);
                                }
                            }
                            (_, Some(Field::DigOut(_))) => {
                                if dir == Some(Direction::Up) {
                                    inner = !inner;
                                } else {
                                    dir = Some(Direction::Down);
                                }
                            }
                            (None, None) | (None, Some(Field::Dirt)) | (Some(Field::Dirt), Some(Field::Dirt)) | (Some(Field::Dirt), None) => {}
                        }
                    }
                }
            }
        }
        println!("{:?}", grid);

        grid.fields.iter().flat_map(|row| row.iter()).filter(|f| {
            match f {
                Field::DigOut(_) => true,
                _ => false,
            }
        }).count()
    }

    /// Count the number of inner dirt fields.
    /// Here it is done through a flood fill search.
    fn count_inner_dirt_flood(&self) -> usize {
        0
    }
}

fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    println!("{:?}", grid);
    grid.count_inner_dirt_pixel()
}

fn part2(input: &str) -> usize {
    0
}

pub fn day() -> String {
    let input = include_str!("../input/day18.txt");
    format!("Day 18\tPart 1: {}\tPart 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 62);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
