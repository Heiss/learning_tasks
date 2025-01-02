use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(PartialEq, Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    fn up(&self) -> Self {
        Point::new(self.x, self.y - 1)
    }

    fn down(&self) -> Self {
        Point::new(self.x, self.y + 1)
    }

    fn left(&self) -> Self {
        Point::new(self.x - 1, self.y)
    }

    fn right(&self) -> Self {
        Point::new(self.x + 1, self.y)
    }

    fn get_gps_distance(&self) -> usize {
        self.x + self.y * 100
    }

    fn go_at(&self, direction: &Direction) -> Point {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Direction::Up),
            "v" => Ok(Direction::Down),
            "<" => Ok(Direction::Left),
            ">" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Field {
    Wall,
    Empty,
    Robot,
    Box,
    BoxLeft,
    BoxRight,
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Field::Wall),
            "." => Ok(Field::Empty),
            "@" => Ok(Field::Robot),
            "O" => Ok(Field::Box),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<Field>>,
    robot: Point,
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for field in row {
                let c = match field {
                    Field::Wall => '#',
                    Field::Empty => '.',
                    Field::Robot => '@',
                    Field::Box => 'O',
                    Field::BoxLeft => '[',
                    Field::BoxRight => ']',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn move_object(&mut self, position: &Point, direction: &Direction) -> bool {
        let new_position = position.go_at(direction);
        if new_position.x >= self.map[0].len() - 1
            || new_position.y >= self.map.len() - 1
            || position.x == 0
            || position.y == 0
        {
            return false;
        }
        let current_object = self.map[position.y][position.x];
        let next_object = self.map[new_position.y][new_position.x];

        let next_is_empty = match (next_object, direction) {
            (Field::Empty, _) => true,
            (Field::Wall, _) => false,
            (_, Direction::Left) | (_, Direction::Right) | (Field::Box, _) => {
                self.move_object(&new_position, direction)
            }
            (Field::Robot, _) => panic!("Robot should not be found in the map"),
            (v, Direction::Down) | (v, Direction::Up) => {
                let left = if v == Field::BoxRight {
                    new_position.left()
                } else {
                    new_position.right()
                };
                self.move_object(&left, direction) && self.move_object(&new_position, direction)
            }
        };

        if next_is_empty {
            self.map[position.y][position.x] = Field::Empty;
            self.map[new_position.y][new_position.x] = current_object;
        }

        next_is_empty
    }

    fn gps_coords(&self) -> Vec<Point> {
        let mut coords = Vec::new();
        for (y, row) in self.map.iter().enumerate() {
            for (x, field) in row.iter().enumerate() {
                if *field == Field::Box || *field == Field::BoxLeft {
                    coords.push(Point::new(x, y));
                }
            }
        }
        coords
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut robot = Point::new(0, 0);
        let mut map = Vec::new();

        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.char_indices() {
                let current_field = Field::from_str(&c.to_string())?;
                if current_field == Field::Robot {
                    robot = Point::new(x, y);
                }
                row.push(current_field);
            }
            map.push(row);
        }
        Ok(Map { map, robot })
    }
}

#[derive(Debug)]
struct Warehouse {
    map: Map,
    directions: Vec<Direction>,
}

impl Warehouse {
    fn move_robot(&mut self) -> &mut Self {
        for direction in &self.directions {
            let old_map = self.map.clone();
            if self.map.move_object(&self.map.robot.clone(), direction) {
                self.map.robot = self.map.robot.go_at(direction);
            } else {
                self.map = old_map
            }
        }

        self
    }

    fn sum_gps_coords(&self) -> usize {
        self.map
            .gps_coords()
            .iter()
            .map(|p| p.get_gps_distance())
            .sum()
    }

    fn stretch_map(&mut self) -> &mut Self {
        let mut new_map = vec![vec![Field::Empty; self.map.map[0].len() * 2]; self.map.map.len()];

        for (y, row) in self.map.map.iter().enumerate() {
            for (x, field) in row.iter().enumerate() {
                let new_field: Vec<Field> = match field {
                    Field::Wall => {
                        vec![Field::Wall, Field::Wall]
                    }
                    Field::Empty => {
                        vec![Field::Empty, Field::Empty]
                    }
                    Field::Robot => {
                        vec![Field::Robot, Field::Empty]
                    }
                    Field::Box => {
                        vec![Field::BoxLeft, Field::BoxRight]
                    }
                    Field::BoxLeft | Field::BoxRight => panic!("WideBox should not be in the map"),
                };
                new_map[y][x * 2] = new_field[0];
                new_map[y][x * 2 + 1] = new_field[1];
            }
        }

        let mut robot = self.map.robot;
        for (y, row) in new_map.iter().enumerate() {
            for (x, field) in row.iter().enumerate() {
                if let Field::Robot = field {
                    robot = Point::new(x, y);
                }
            }
        }

        self.map = Map {
            map: new_map,
            robot,
        };

        self
    }
}

impl FromStr for Warehouse {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (m, dir) = s.split_once("\n\n").unwrap();

        Ok(Warehouse {
            map: Map::from_str(m)?,
            directions: dir
                .chars()
                .filter(|c| *c != '\n')
                .map(|c| Direction::from_str(&c.to_string()).unwrap())
                .collect(),
        })
    }
}

fn part1(input: &str) -> usize {
    Warehouse::from_str(input)
        .unwrap()
        .move_robot()
        .sum_gps_coords()
}

fn part2(input: &str) -> usize {
    Warehouse::from_str(input)
        .unwrap()
        .stretch_map()
        .move_robot()
        .sum_gps_coords()
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode24/day15.txt");
    format!(
        "Day 15\tPart 1: {}\t Part 2: {}",
        part1(input),
        part2(input)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
    #[test]
    fn day1() {
        assert_eq!(
            part1(
                r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#
            ),
            2028
        );
        assert_eq!(part1(INPUT), 10092);
    }
    #[test]
    fn day2() {
        assert_eq!(
            part2(
                r#"########
#......#
#..O...#
#.@OO..#
#..#O..#
#......#
#......#
########

>><^>>^>v"#
            ),
            1232
        );
        assert_eq!(part2(INPUT), 9021);
    }
}
