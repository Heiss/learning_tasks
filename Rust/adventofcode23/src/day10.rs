use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Status {
    NotLoop,
    Loop(Pipe),
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    UpToDown,
    DownToUp,
    LeftToRight,
    RightToLeft,
}

impl Direction {
    fn get_opposite(&self) -> Direction {
        match self {
            Direction::UpToDown => Direction::DownToUp,
            Direction::DownToUp => Direction::UpToDown,
            Direction::LeftToRight => Direction::RightToLeft,
            Direction::RightToLeft => Direction::LeftToRight,
        }
    }
}

impl Direction {
    fn get_point(&self) -> Point {
        match self {
            Direction::UpToDown => Point::new(0, -1),
            Direction::DownToUp => Point::new(0, 1),
            Direction::LeftToRight => Point::new(-1, 0),
            Direction::RightToLeft => Point::new(1, 0),
        }
    }
}
const VALID_DIRECTIONS: [Direction; 4] = [
    Direction::UpToDown,
    Direction::DownToUp,
    Direction::LeftToRight,
    Direction::RightToLeft,
];

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl From<Point> for Direction {
    fn from(value: Point) -> Self {
        match value {
            Point { x: 0, y: -1 } => Direction::UpToDown,
            Point { x: 0, y: 1 } => Direction::DownToUp,
            Point { x: -1, y: 0 } => Direction::LeftToRight,
            Point { x: 1, y: 0 } => Direction::RightToLeft,
            _ => panic!("Invalid point: {:?}", value),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Field {
    StartingPoint,
    Ground,
    Pipe(Pipe),
}

const UP: Point = Point { x: 0, y: -1 };
const DOWN: Point = Point { x: 0, y: 1 };
const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Field::StartingPoint),
            "." => Ok(Field::Ground),
            "|" | "-" | "L" | "J" | "7" | "F" => Ok(Field::Pipe(s.parse().unwrap())),
            _ => panic!("Invalid field: {:?}", s),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pipe {
    UpDown,
    LeftRight,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

const ALL_PIPES: [Pipe; 6] = [
    Pipe::UpDown,
    Pipe::LeftRight,
    Pipe::UpRight,
    Pipe::UpLeft,
    Pipe::DownRight,
    Pipe::DownLeft,
];

impl Pipe {
    fn get_next_pipe_direction(&self, from_direction: &Direction) -> Point {
        match (self, from_direction) {
            (Self::UpDown, Direction::UpToDown)
            | (Self::DownRight, Direction::RightToLeft)
            | (Self::DownLeft, Direction::LeftToRight) => DOWN,
            (Self::UpDown, Direction::DownToUp)
            | (Self::UpRight, Direction::RightToLeft)
            | (Self::UpLeft, Direction::LeftToRight) => UP,
            (Self::LeftRight, Direction::LeftToRight)
            | (Self::UpRight, Direction::UpToDown)
            | (Self::DownRight, Direction::DownToUp) => RIGHT,
            (Self::LeftRight, Direction::RightToLeft)
            | (Self::UpLeft, Direction::UpToDown)
            | (Self::DownLeft, Direction::DownToUp) => LEFT,
            _ => panic!("Invalid pipe: {:?} from {:?}", self, from_direction),
        }
    }

    fn get_pipe_from_two_directions(inbound: &Direction, outbound: &Direction) -> Self {
        match (inbound, outbound) {
            (Direction::UpToDown, Direction::LeftToRight)
            | (Direction::LeftToRight, Direction::DownToUp) => Self::UpRight,
            (Direction::UpToDown, Direction::RightToLeft)
            | (Direction::RightToLeft, Direction::DownToUp) => Self::UpLeft,
            (Direction::DownToUp, Direction::LeftToRight)
            | (Direction::LeftToRight, Direction::UpToDown) => Self::DownRight,
            (Direction::DownToUp, Direction::RightToLeft)
            | (Direction::RightToLeft, Direction::UpToDown) => Self::DownLeft,
            (Direction::UpToDown, Direction::UpToDown)
            | (Direction::DownToUp, Direction::DownToUp) => Self::UpDown,
            (Direction::LeftToRight, Direction::LeftToRight)
            | (Direction::RightToLeft, Direction::RightToLeft) => Self::LeftRight,
            _ => panic!("Invalid directions: {:?} and {:?}", inbound, outbound),
        }
    }

    fn is_valid(&self, from_direction: &Direction) -> bool {
        match (self, from_direction) {
            (Pipe::UpDown, Direction::UpToDown) => true,
            (Pipe::UpDown, Direction::DownToUp) => true,
            (Pipe::LeftRight, Direction::LeftToRight) => true,
            (Pipe::LeftRight, Direction::RightToLeft) => true,
            (Pipe::UpRight, Direction::RightToLeft) => true,
            (Pipe::UpRight, Direction::UpToDown) => true,
            (Pipe::UpLeft, Direction::UpToDown) => true,
            (Pipe::UpLeft, Direction::LeftToRight) => true,
            (Pipe::DownRight, Direction::DownToUp) => true,
            (Pipe::DownRight, Direction::RightToLeft) => true,
            (Pipe::DownLeft, Direction::LeftToRight) => true,
            (Pipe::DownLeft, Direction::DownToUp) => true,
            _ => false,
        }
    }
}

impl FromStr for Pipe {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Pipe::UpDown),
            "-" => Ok(Pipe::LeftRight),
            "L" => Ok(Pipe::UpRight),
            "J" => Ok(Pipe::UpLeft),
            "7" => Ok(Pipe::DownLeft),
            "F" => Ok(Pipe::DownRight),
            _ => panic!("Invalid pipe: {}", s),
        }
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Field>>,
    starting_point: Point,
}

impl Map {
    fn set_field(&mut self, p0: &Point, p1: Field) {
        self.map[p0.y as usize][p0.x as usize] = p1;
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        let mut starting_point = None;
        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, field) in line.chars().enumerate() {
                let field = field.to_string().parse()?;
                if let Field::StartingPoint = field {
                    starting_point = Some(Point::new(x as isize, y as isize));
                }
                row.push(field);
            }
            map.push(row);
        }

        Ok(Self {
            map,
            starting_point: starting_point.expect("No starting point found"),
        })
    }
}

impl Map {
    fn get_field(&self, point: &Point) -> Option<&Field> {
        self.map
            .get(point.y as usize)
            .and_then(|row| row.get(point.x as usize))
    }

    fn get_iterators(&self) -> Vec<MapIterator> {
        let all_combinations: Vec<_> = VALID_DIRECTIONS
            .iter()
            .flat_map(|d| ALL_PIPES.iter().map(move |&p| (d, p)))
            .filter(|(d, p)| {
                let res = p.is_valid(d);
                res
            })
            .filter_map(|(d, p)| {
                let d = d.get_opposite();
                let next = d.get_point().add(&self.starting_point);
                if let Some(Field::Pipe(p2)) = self.get_field(&next) {
                    if p == *p2 {
                        return Some((d, p));
                    }
                }
                None
            })
            .map(|(d, _)| MapIterator {
                map: self,
                current_point: self.starting_point.add(&d.get_point()),
                coming_from_direction: d.get_opposite(),
            })
            .collect();

        assert_eq!(all_combinations.len(), 2);
        all_combinations
    }

    fn steps_to_farthest_position(&self) -> usize {
        let mut steps = 1;
        let mut iterators = self.get_iterators();

        while iterators
            .windows(2)
            .all(|w| w[0].current_point != w[1].current_point)
        {
            steps += 1;
            iterators = iterators
                .into_iter()
                .filter_map(|mut i| i.next().map(|_| i))
                .collect();
            assert!(iterators.len() >= 2);
        }

        steps
    }

    fn count_enclosed_tiles(&self) -> usize {
        let mut iterators = self.get_iterators();
        let mut new_map: Vec<Vec<Status>> =
            vec![vec![Status::NotLoop; self.map[0].len()]; self.map.len()];

        if let Some(Field::Pipe(pipe)) = self.get_field(&self.starting_point) {
            new_map[self.starting_point.y as usize][self.starting_point.x as usize] =
                Status::Loop(*pipe);
        } else {
            panic!("Invalid starting point");
        }

        iterators.iter().for_each(|i| {
            self.set_status(&mut new_map, i);
        });

        while iterators
            .windows(2)
            .all(|w| w[0].current_point != w[1].current_point)
        {
            iterators = iterators
                .into_iter()
                .filter_map(|mut i| {
                    i.next().map(|_| {
                        self.set_status(&mut new_map, &i);
                        i
                    })
                })
                .collect();
            assert!(iterators.len() >= 2);
        }

        new_map
            .iter()
            .map(|line| {
                let mut total = 0;
                let mut in_loop = false;

                for field in line {
                    match field {
                        Status::NotLoop => {
                            total += in_loop as usize;
                        }
                        Status::Loop(Pipe::UpDown)
                        | Status::Loop(Pipe::DownRight)
                        | Status::Loop(Pipe::DownLeft) => {
                            in_loop = !in_loop;
                        }
                        _ => {}
                    }
                }

                total
            })
            .sum()
    }

    fn set_status(&self, new_map: &mut Vec<Vec<Status>>, i: &MapIterator) {
        new_map[i.current_point.y as usize][i.current_point.x as usize] = Status::Loop(
            *self
                .get_field(&i.current_point)
                .and_then(|f| match f {
                    Field::Pipe(p) => Some(p),
                    _ => None,
                })
                .expect("Invalid field"),
        );
    }
}

#[derive(Debug)]
struct MapIterator<'a> {
    map: &'a Map,
    current_point: Point,
    coming_from_direction: Direction,
}

impl Deref for MapIterator<'_> {
    type Target = Map;

    fn deref(&self) -> &Self::Target {
        self.map
    }
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self
            .map
            .get_field(&self.current_point)
            .and_then(|f| match f {
                Field::Pipe(p) => Some(p.get_next_pipe_direction(&self.coming_from_direction)),
                _ => None,
            })
            .map(|p| self.current_point.add(&p));

        if let Some(next) = next {
            self.coming_from_direction = self.current_point.sub(&next).into();
            self.current_point = next;
        }

        next
    }
}

fn part1(input: &str) -> usize {
    let map = input.parse::<Map>().unwrap();
    map.steps_to_farthest_position()
}

fn part2(input: &str) -> usize {
    let mut map = input.parse::<Map>().unwrap();
    let ds = map
        .get_iterators()
        .iter()
        .map(|i| i.coming_from_direction)
        .collect::<Vec<_>>();

    let d = Pipe::get_pipe_from_two_directions(&ds[0].get_opposite(), &ds[1]);
    map.set_field(&map.starting_point.clone(), Field::Pipe(d));

    map.count_enclosed_tiles()
}

pub fn day() {
    let input = include_str!("../input/day10.txt");
    print!("Day 10\t");
    print!("Part 1: {}\t", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                r#".....
.S-7.
.|.|.
.L-J.
....."#
            ),
            4
        );
        assert_eq!(
            part1(
                r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#
            ),
            8
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#
            ),
            4
        );
        assert_eq!(
            part2(
                r#"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."#
            ),
            4
        );
        assert_eq!(
            part2(
                r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#
            ),
            8
        );
        assert_eq!(
            part2(
                r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#
            ),
            10
        );
    }

    #[test]
    fn test_bool() {
        assert_eq!(false as usize, 0);
        assert_eq!(true as usize, 1);
    }
}
