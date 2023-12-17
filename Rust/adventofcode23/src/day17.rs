use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Debug;
use std::ops::Range;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Number::One),
            "2" => Ok(Number::Two),
            "3" => Ok(Number::Three),
            "4" => Ok(Number::Four),
            "5" => Ok(Number::Five),
            "6" => Ok(Number::Six),
            "7" => Ok(Number::Seven),
            "8" => Ok(Number::Eight),
            "9" => Ok(Number::Nine),
            _ => Err(()),
        }
    }
}

impl Number {
    fn get_value(&self) -> usize {
        match self {
            Number::One => 1,
            Number::Two => 2,
            Number::Three => 3,
            Number::Four => 4,
            Number::Five => 5,
            Number::Six => 6,
            Number::Seven => 7,
            Number::Eight => 8,
            Number::Nine => 9,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    fields: Vec<Vec<Number>>,
}

impl Grid {
    fn get(&self, pos: &Point) -> Number {
        self.fields[pos.y as usize][pos.x as usize]
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '1'..='9' => Number::from_str(&c.to_string()).unwrap(),
                        _ => panic!("invalid input {c}"),
                    })
                    .collect()
            })
            .collect();

        Ok(Self { fields: grid })
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, Ord, PartialOrd)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_point(&self) -> Point {
        match self {
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn straight(&self) -> Direction {
        self.clone()
    }

    fn get_next_directions(
        &self,
        steps: &usize,
        min_forward: &usize,
        max_forward: &usize,
    ) -> Vec<Direction> {
        if *steps < *min_forward {
            vec![self.clone()]
        } else if *steps < *max_forward {
            vec![self.turn_left(), self.straight(), self.turn_right()]
        } else {
            vec![self.turn_left(), self.turn_right()]
        }
    }
}

type Steps = usize;
type Distance = usize;

#[derive(Debug, Clone, Hash)]
struct Step {
    pos: Point,
    direction: Direction,
    steps_in_same_direction: Steps,
    distance: Distance,
}

impl Eq for Step {}

impl PartialEq<Self> for Step {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
            && self.direction == other.direction
            && self.steps_in_same_direction == other.steps_in_same_direction
    }
}

impl PartialOrd<Self> for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl Step {
    fn new(pos: Point, direction: Direction, steps: Steps, distance: Distance) -> Self {
        Self {
            pos,
            direction,
            steps_in_same_direction: steps,
            distance,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct VisitedKey {
    pos: Point,
    direction: Direction,
    steps_in_same_direction: Steps,
}

impl From<Step> for VisitedKey {
    fn from(value: Step) -> Self {
        Self {
            pos: value.pos,
            direction: value.direction,
            steps_in_same_direction: value.steps_in_same_direction,
        }
    }
}

type DistancePath = Option<Step>;

struct Crucible<'a> {
    grid: &'a Grid,
}

impl<'a> Crucible<'a> {
    fn new(grid: &'a Grid) -> Self {
        Self { grid }
    }

    fn calculates_distances(
        &self,
        point: Point,
        dest: Point,
        min_forward: usize,
        max_forward: usize,
    ) -> Option<Step> {
        let mut queue: BinaryHeap<Step> = BinaryHeap::new();
        let mut distances: HashMap<VisitedKey, Distance> =
            HashMap::with_capacity(self.grid.fields.len() * self.grid.fields[0].len());

        let step = Step {
            pos: point,
            direction: Direction::Right,
            steps_in_same_direction: 0,
            distance: 0,
        };

        distances.insert(step.clone().into(), 0);
        queue.push(step);

        while let Some(current_step) = queue.pop() {
            if current_step.pos == dest && current_step.steps_in_same_direction >= min_forward {
                return Some(current_step);
            }
            if let Some(d) = distances.get(&current_step.clone().into()) {
                if current_step.distance > *d {
                    continue;
                }
            }

            let next_directions = current_step.direction.get_next_directions(
                &current_step.steps_in_same_direction,
                &min_forward,
                &max_forward,
            );
            for dir in next_directions {
                let next_pos = current_step.pos.add(&dir.get_point());
                if next_pos.x < 0
                    || next_pos.y < 0
                    || next_pos.y >= self.grid.fields.len() as isize
                    || next_pos.x >= self.grid.fields[0].len() as isize
                {
                    continue;
                }

                let next_number = self.grid.get(&next_pos);
                let next_distance = current_step.distance + next_number.get_value();
                let next_step_in_same_direction = if dir == current_step.direction {
                    current_step.steps_in_same_direction + 1
                } else {
                    1
                };

                let next_step =
                    Step::new(next_pos, dir, next_step_in_same_direction, next_distance);

                let dist_key = next_step.clone().into();
                let mut d = distances.get_mut(&dist_key);
                if next_distance < d.as_ref().map(|s| **s).unwrap_or(usize::MAX) {
                    if distances.insert(dist_key, next_distance).is_none() {
                        queue.push(next_step.clone());
                    }
                }
            }
        }

        None
    }

    fn get_minimum_distance(&mut self, p: &Point, Range { start, end }: Range<usize>) -> usize {
        let distances = self.calculates_distances(Point { x: 0, y: 0 }, p.clone(), start, end);

        let mut s = distances.unwrap();
        let res = s.distance;

        res
    }
}

fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    let mut crucible = Crucible::new(&grid);

    crucible.get_minimum_distance(
        &Point {
            x: (grid.fields[0].len() - 1) as isize,
            y: (grid.fields.len() - 1) as isize,
        },
        0..3,
    )
}

fn part2(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    let mut crucible = Crucible::new(&grid);

    crucible.get_minimum_distance(
        &Point {
            x: (grid.fields[0].len() - 1) as isize,
            y: (grid.fields.len() - 1) as isize,
        },
        4..10,
    )
}

pub fn day() -> String {
    let input = include_str!("../input/day17.txt");
    format!("Day 17\tPart 1: {}\tPart 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1("1111\n9991\n9991\n9991"), 6);
        assert_eq!(part1("1999\n1999\n1999\n1111"), 6);
        assert_eq!(part1(INPUT), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 94);
        assert_eq!(
            part2(
                r#"111111111111
999999999991
999999999991
999999999991
999999999991"#
            ),
            71
        );
    }
}
