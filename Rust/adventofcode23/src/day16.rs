use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Field {
    EmptySpace,
    MirrorBackslash,
    MirrorSlash,
    SplitterPipe,
    SplitterDash,
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::EmptySpace),
            "\\" => Ok(Self::MirrorBackslash),
            "/" => Ok(Self::MirrorSlash),
            "|" => Ok(Self::SplitterPipe),
            "-" => Ok(Self::SplitterDash),
            _ => panic!("invalid char {s:?}"),
        }
    }
}

impl Field {
    fn encounter_from_direction(&self, b: &Beam) -> Vec<Beam> {
        let mut beams = Vec::new();

        match (self, &b.direction) {
            (Field::MirrorSlash, Direction::Left) => {
                beams.push(Beam {
                    position: b.position.add_direction(&Direction::Down),
                    direction: Direction::Down,
                });
            }
            (Field::MirrorSlash, Direction::Right) => {
                beams.push(Beam {
                    position: b.position.add_direction(&Direction::Up),
                    direction: Direction::Up,
                });
            }
            (Field::MirrorSlash, Direction::Down) => {
                beams.push(Beam {
                    position: b.position.add_direction(&Direction::Left),
                    direction: Direction::Left,
                });
            }
            (Field::MirrorSlash, Direction::Up) => {
                beams.push(Beam {
                    position: b.position.add_direction(&Direction::Right),
                    direction: Direction::Right,
                });
            }
            (Self::MirrorBackslash, Direction::Up) => {
                beams.push(Beam {
                    position: b.position.add_direction(&Direction::Left),
                    direction: Direction::Left,
                });
            }
            (Self::MirrorBackslash, Direction::Down) => {
                beams.push(Beam {
                    position: b.position.add_direction(&Direction::Right),
                    direction: Direction::Right,
                });
            }
            (Self::MirrorBackslash, Direction::Left) => {
                beams.push(Beam {
                    position: b.position.add_direction(&Direction::Up),
                    direction: Direction::Up,
                });
            }
            (Self::MirrorBackslash, Direction::Right) => {
                beams.push(Beam {
                    position: b.position.add_direction(&Direction::Down),
                    direction: Direction::Down,
                });
            }
            (Field::SplitterPipe, Direction::Left) | (Field::SplitterPipe, Direction::Right) => {
                beams.push(Beam {
                    position: b.position.add(&Direction::Up.get_point()),
                    direction: Direction::Up,
                });
                beams.push(Beam {
                    position: b.position.add(&Direction::Down.get_point()),
                    direction: Direction::Down,
                });
            }
            (Field::SplitterDash, Direction::Down) | (Field::SplitterDash, Direction::Up) => {
                beams.push(Beam {
                    position: b.position.add_direction(&Direction::Left),
                    direction: Direction::Left,
                });
                beams.push(Beam {
                    position: b.position.add_direction(&Direction::Right),
                    direction: Direction::Right,
                });
            }
            (Self::EmptySpace, _) | (Self::SplitterPipe, _) | (Self::SplitterDash, _) => beams
                .push(Beam {
                    position: b.position.add_direction(&b.direction),
                    direction: b.direction,
                }),
            _ => panic!("invalid encounter Field {self:?} Beam {b:?}"),
        }

        beams
    }
}

#[derive(Hash, Copy, Clone)]
struct Beam {
    position: Point,
    direction: Direction,
}

impl PartialEq for Beam {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.direction == other.direction
    }
}

impl Eq for Beam {}

impl Debug for Beam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.position, self.direction)
    }
}

#[derive(Hash, Copy, Clone, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
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

    fn add_direction(&self, other: &Direction) -> Self {
        Self {
            x: self.x + other.get_point().x,
            y: self.y + other.get_point().y,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Debug for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "U"),
            Self::Down => write!(f, "D"),
            Self::Left => write!(f, "L"),
            Self::Right => write!(f, "R"),
        }
    }
}

impl Direction {
    fn get_point(&self) -> Point {
        match self {
            Self::Up => Point::new(0, -1),
            Self::Down => Point::new(0, 1),
            Self::Left => Point::new(-1, 0),
            Self::Right => Point::new(1, 0),
        }
    }
}

#[derive(Clone)]
struct Grid {
    fields: Vec<Vec<Field>>,
    energized_fields: HashSet<Beam>,
    beams: Vec<Beam>,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.fields.iter().enumerate() {
            for (x, field) in row.iter().enumerate() {
                match field {
                    Field::MirrorBackslash => write!(f, "\\")?,
                    Field::MirrorSlash => write!(f, "/")?,
                    Field::SplitterPipe => write!(f, "|")?,
                    Field::SplitterDash => write!(f, "-")?,
                    Field::EmptySpace => {
                        let pos = Point::new(x as isize, y as isize);

                        let count = self
                            .energized_fields
                            .iter()
                            .filter(|b| b.position == pos)
                            .count();
                        if count > 1 {
                            write!(f, "{}", count)?;
                        } else if self.energized_fields.contains(&Beam {
                            position: pos,
                            direction: Direction::Up,
                        }) {
                            write!(f, "^")?;
                        } else if self.energized_fields.contains(&Beam {
                            position: pos,
                            direction: Direction::Down,
                        }) {
                            write!(f, "v")?;
                        } else if self.energized_fields.contains(&Beam {
                            position: pos,
                            direction: Direction::Left,
                        }) {
                            write!(f, "<")?;
                        } else if self.energized_fields.contains(&Beam {
                            position: pos,
                            direction: Direction::Right,
                        }) {
                            write!(f, ">")?;
                        } else {
                            write!(f, ".")?;
                        }
                    }
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
        let mut fields = Vec::new();

        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(Field::from_str(&c.to_string()).unwrap());
            }
            fields.push(row);
        }

        let mut energized_fields = HashSet::new();
        energized_fields.insert(Beam {
            position: Point::new(0, 0),
            direction: Direction::Right,
        });

        Ok(Self {
            fields,
            beams: vec![Beam {
                position: Point::new(0, 0),
                direction: Direction::Right,
            }],
            energized_fields,
        })
    }
}

impl Grid {
    fn proceed_encounter(&mut self) {
        let mut new_beams = Vec::new();

        for beam in &self.beams {
            let field = &self.fields[beam.position.y as usize][beam.position.x as usize];

            let mut next_beams = field.encounter_from_direction(&beam);
            next_beams.retain(|b| {
                b.position.x >= 0
                    && b.position.y >= 0
                    && b.position.x < self.fields[0].len() as isize
                    && b.position.y < self.fields.len() as isize
            });

            for b in next_beams {
                if self.energized_fields.insert(b) {
                    new_beams.push(b);
                }
            }
        }

        self.beams = new_beams;
    }

    fn count_energized_fields(&self) -> usize {
        self.energized_fields
            .iter()
            .fold(Vec::new(), |mut acc, v| {
                if !acc
                    .iter()
                    .find_map(|b: &Beam| {
                        if b.position == v.position {
                            Some(b)
                        } else {
                            None
                        }
                    })
                    .is_some()
                {
                    acc.push(v.clone());
                }
                acc
            })
            .len()
    }
}

fn part1(input: &str) -> usize {
    let mut grid = Grid::from_str(input).unwrap();

    while grid.beams.len() > 0 {
        grid.proceed_encounter();
    }

    grid.count_energized_fields()
}

fn part2(input: &str) -> usize {
    let mut grid = Grid::from_str(input).unwrap();
    let energized_fields: HashMap<Beam, usize> = HashMap::new();

    let left = (0..grid.fields.len()).map(|i| Beam {
        position: Point::new(0, i as isize),
        direction: Direction::Right,
    });
    let top = (0..grid.fields[0].len()).map(|i| Beam {
        position: Point::new(i as isize, 0),
        direction: Direction::Down,
    });
    let right = (0..grid.fields.len()).map(|i| Beam {
        position: Point::new(grid.fields[0].len() as isize - 1, i as isize),
        direction: Direction::Left,
    });
    let bottom = (0..grid.fields[0].len()).map(|i| Beam {
        position: Point::new(i as isize, grid.fields.len() as isize - 1),
        direction: Direction::Up,
    });

    let beams: Vec<_> = left.chain(top).chain(right).chain(bottom).collect();

    std::thread::scope(|t| {
        let ts: Vec<_> = beams
            .iter()
            .map(|b| {
                let mut grid = grid.clone();
                let beam = b.clone();
                t.spawn(move || {
                    grid.beams = vec![beam];
                    grid.energized_fields = HashSet::new();
                    grid.energized_fields.insert(beam);

                    while grid.beams.len() > 0 {
                        grid.proceed_encounter();
                    }

                    grid.count_energized_fields()
                })
            })
            .collect();

        ts.into_iter().map(|t| t.join().unwrap()).max().unwrap()
    })
}

pub fn day() {
    let input = include_str!("../input/day16.txt");
    print!("day 16\t");
    print!("part 1: {}\t", part1(input));
    print!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 51);
    }
}
