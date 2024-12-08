use itertools::Itertools;
use std::cmp::PartialEq;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(PartialEq, Clone, Debug)]
enum Object {
    Unvisited,
    Visited(Direction),
    Box,
    Guard,
}

impl FromStr for Object {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "." => Object::Unvisited,
            "#" => Object::Box,
            "^" => Object::Guard,
            v => panic!("Wrong char: {}", v),
        })
    }
}

#[derive(Clone, Hash, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    AllDir,
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::Up, Direction::Up)
            | (Direction::Down, Direction::Down)
            | (Direction::Left, Direction::Left)
            | (Direction::Right, Direction::Right)
            | (Direction::AllDir, _)
            | (_, Direction::AllDir) => true,
            _ => false,
        }
    }
}

type Point = (isize, isize);

impl Direction {
    fn next_field(&self) -> Point {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::AllDir => panic!("Should not happen!"),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::AllDir => panic!("Should not happen!"),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Guard {
    fn next_field(&self) -> Point {
        let next_field = self.direction.next_field();
        (
            self.x as isize + next_field.0,
            self.y as isize + next_field.1,
        )
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    fn set_position(&mut self, p: Point) {
        self.x = p.0 as usize;
        self.y = p.1 as usize;
    }

    fn peek<'a>(&self, map: &'a Grid) -> Option<(&'a Object, Point)> {
        let next_field = self.next_field();
        if next_field.1 < 0
            || next_field.0 < 0
            || next_field.1 as usize >= map.map.len()
            || next_field.0 as usize >= map.map[0].len()
        {
            None
        } else {
            Some((
                &map.map[next_field.1 as usize][next_field.0 as usize],
                next_field,
            ))
        }
    }
    fn walk(&mut self, map: &mut Grid) -> Option<Object> {
        let v = self.peek(map)?;

        match v {
            (Object::Box, _) => {
                self.turn_right();
                Some(v.0.clone())
            }
            (Object::Visited(_d), v) => {
                self.set_position(v);
                map.map[self.y][self.x] = Object::Visited(Direction::AllDir);
                Some(Object::Visited(Direction::AllDir))
            }
            (Object::Unvisited, v) => {
                self.set_position(v);
                map.map[self.y][self.x] = Object::Visited(self.direction.clone());
                Some(Object::Visited(self.direction.clone()))
            }
            _ => None,
        }
    }
}

#[derive(Clone)]
struct Grid {
    guard: Guard,
    map: Vec<Vec<Object>>,
}

impl Grid {
    fn walk(&mut self) -> Option<Object> {
        let mut guard = self.guard.clone();
        let res = guard.walk(self);
        self.guard = guard;
        res
    }

    fn peek(&self) -> Option<(&Object, Point)> {
        self.guard.peek(self)
    }

    fn is_a_loop_on_the_right(&self, seen_positions: &HashSet<Guard>, mut map: Grid) -> bool {
        let mut seen_positions = seen_positions.clone();
        map.guard.turn_right();

        while let Some(_) = map.walk() {
            if !seen_positions.insert(map.guard.clone()) {
                return true;
            }
        }

        false
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut guard = None;
        let map = s
            .lines()
            .enumerate()
            .map(|v| {
                v.1.chars()
                    .enumerate()
                    .map(|l| {
                        let mut o = Object::from_str(&l.1.to_string()).unwrap();
                        if o == Object::Guard {
                            guard = Some(Guard {
                                x: l.0,
                                y: v.0,
                                direction: Direction::Up,
                            });
                            o = Object::Visited(Direction::Up);
                        }
                        o
                    })
                    .collect()
            })
            .collect();
        Ok(Self {
            map,
            guard: guard.unwrap(),
        })
    }
}

fn part1(input: &str) -> usize {
    let mut grid = Grid::from_str(input).unwrap();
    while let Some(_g) = grid.walk() {}
    grid.map
        .iter()
        .flat_map(|m| m)
        .filter(|&m| match m {
            Object::Visited(_) => true,
            _ => false,
        })
        .count()
}

fn part2(input: &str) -> usize {
    let mut grid = Grid::from_str(input).unwrap();
    let init = grid.guard.clone();
    let mut obstacle_positions = HashSet::new();
    let mut visited_positions = HashSet::new();

    while let Some((o, p)) = grid.peek() {
        visited_positions.insert(grid.guard.clone());
        let mut m = grid.clone();
        m.map[p.1 as usize][p.0 as usize] = Object::Box;
        if *o != Object::Box && grid.is_a_loop_on_the_right(&visited_positions, m) {
            let p = (p.0 as usize, p.1 as usize);
            if !visited_positions.iter().map(|g| (g.x, g.y)).contains(&p) {
                obstacle_positions.insert(p);
            }
        }
        grid.walk();
    }

    obstacle_positions.remove(&(init.x, init.y));
    /* For output purposes only
    for y in 0..grid.map.len() {
        let mut res = String::new();
        for x in 0..grid.map[0].len() {
            if init.x == x && init.y == y {
                res.push('X');
            } else if obstacle_positions.contains(&(x, y)) {
                res.push('O');
            } else if grid.guard.x == x && grid.guard.y == y {
                res.push(match grid.guard.direction {
                    Direction::Up => '^',
                    Direction::Down => 'v',
                    Direction::Left => '<',
                    Direction::Right => '>',
                    Direction::AllDir => panic!("Should not be here!"),
                });
            } else {
                match &grid.map[y][x] {
                    Object::Unvisited => res.push('.'),
                    Object::Visited(v) => res.push(match v {
                        Direction::Up | Direction::Down => '|',
                        Direction::Left | Direction::Right => '-',
                        Direction::AllDir => '+',
                    }),
                    Object::Box => res.push('#'),
                    _ => panic!("Should not be here."),
                }
            }
        }
        println!("{}", res);
    }*/

    obstacle_positions.len()
}

pub fn day() -> String {
    let input = include_str!("../input/day6.txt");
    format!("Day 6\tPart 1: {}\t Part 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
    #[test]
    fn day1() {
        assert_eq!(part1(INPUT), 41);
    }
    #[test]
    fn day2() {
        assert_eq!(part2(INPUT), 6);
        assert_eq!(
            part2(
                r#"....
#..#
.^#."#
            ),
            1
        );
        assert_eq!(
            (part2(
                r#"###
#.#
#.#
#^#"#
            )),
            0
        );
        assert_eq!(
            part2(
                r#"....
#...
.^#.
.#.."#
            ),
            0
        );
        assert_eq!(
            part2(
                r#"###
..#
^##"#
            ),
            0
        )
    }
}
