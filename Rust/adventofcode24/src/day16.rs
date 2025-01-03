use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn get_point(&self, point: &Point) -> Point {
        match self {
            Direction::Up => Point {
                x: point.x,
                y: point.y - 1,
            },
            Direction::Down => Point {
                x: point.x,
                y: point.y + 1,
            },
            Direction::Left => Point {
                x: point.x - 1,
                y: point.y,
            },
            Direction::Right => Point {
                x: point.x + 1,
                y: point.y,
            },
        }
    }

    fn get_neighbours(&self, point: &Point) -> Vec<(Point, Direction, Cost)> {
        vec![
            (self.get_point(point), *self, 1),
            (*point, self.rotate_left(), 1000),
            (*point, self.rotate_right(), 1000),
        ]
    }

    fn all_directions() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

type Cost = usize;

#[derive(Clone, Copy, PartialEq)]
enum Field {
    Empty,
    Wall,
    Start,
    End,
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Field::Wall),
            "." => Ok(Field::Empty),
            "S" => Ok(Field::Start),
            "E" => Ok(Field::End),
            _ => Err(()),
        }
    }
}

struct Grid {
    grid: Vec<Vec<Field>>,
    distances: HashMap<(Point, Direction), usize>,
    previous: HashMap<(Point, Direction), HashSet<(Point, Direction)>>,
    queue: PriorityQueue<(Point, Direction), Reverse<usize>>,
    start: Point,
    end: Point,
}

impl Grid {
    fn calculate_paths(mut self) -> PathGrid {
        while let Some(((u, d), Reverse(current_distance))) = self.queue.pop() {
            if current_distance > *self.distances.entry((u, d)).or_insert(usize::MAX) {
                continue;
            }
            for (next_pos, dir, next_cost) in d.get_neighbours(&u) {
                if self.grid[next_pos.y][next_pos.x] == Field::Wall {
                    continue;
                }
                let old_distance = self.distances.entry((next_pos, dir)).or_insert(usize::MAX);
                let new_distance = current_distance + next_cost;
                if new_distance <= *old_distance {
                    if new_distance < *old_distance {
                        self.previous.insert((next_pos, dir), HashSet::new());
                        *old_distance = new_distance;
                    }
                    self.previous
                        .entry((next_pos, dir))
                        .or_default()
                        .insert((u, d));
                    self.queue.push((next_pos, dir), Reverse(new_distance));
                }
            }
        }

        PathGrid { grid: self }
    }
}

struct PathGrid {
    grid: Grid,
}

impl Display for PathGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Start {:?}, End {:?}", self.grid.start, self.grid.end)?;
        let mut res: Vec<Vec<char>> =
            vec![vec![' '; self.grid.grid[0].len()]; self.grid.grid.len()];

        let mut path = vec![];
        for d in Direction::all_directions() {
            path.append(&mut self.get_quantum_paths(&self.grid.end, d));
        }

        for y in 0..self.grid.grid.len() {
            for x in 0..self.grid.grid[0].len() {
                let point = Point { x, y };
                let el = self.grid.grid[y][x];
                match el {
                    Field::Wall => res[y][x] = '#',
                    Field::Empty => res[y][x] = '.',
                    _ => {}
                }
                if point == self.grid.start {
                    res[y][x] = 'S';
                } else if point == self.grid.end {
                    res[y][x] = 'E';
                }
            }
        }

        for path in path {
            for p in path {
                res[p.y][p.x] = 'O';
            }
        }

        for row in res {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }

        Ok(())
    }
}

type Path = Vec<Point>;
struct Paths(Vec<Path>);

impl Paths {
    fn count_fields(&self) -> usize {
        let visited: HashSet<Point> = self.0.iter().flatten().cloned().collect();
        visited.len()
    }
}

impl PathGrid {
    fn find_shortest_paths(&self) -> Paths {
        let min_score = self.get_score();
        let mut path_to_end_with_equal_score = vec![];
        for d in Direction::all_directions() {
            if let Some(distance) = self.grid.distances.get(&(self.grid.end, d)) {
                if *distance == min_score {
                    let mut path = self.get_quantum_paths(&self.grid.end, d);
                    path_to_end_with_equal_score.append(&mut path);
                }
            }
        }
        Paths(path_to_end_with_equal_score)
    }

    fn get_quantum_paths(&self, end_point: &Point, end_direction: Direction) -> Vec<Path> {
        let mut all_shortest_paths: Vec<Vec<Point>> = Vec::new();
        let mut stack: VecDeque<(Vec<Point>, (Point, Direction))> = VecDeque::new();
        stack.push_back((vec![*end_point], (*end_point, end_direction)));

        while let Some((current_path, current_node)) = stack.pop_back() {
            if current_node == (self.grid.start, Direction::Right) {
                let mut complete_path = current_path.clone();
                complete_path.reverse();
                all_shortest_paths.push(complete_path);
            } else if let Some(prev_nodes) = self.grid.previous.get(&current_node) {
                for &(prev_pos, prev_dir) in prev_nodes {
                    let mut new_path = current_path.clone();
                    new_path.push(prev_pos);
                    stack.push_back((new_path, (prev_pos, prev_dir)));
                }
            }
        }

        all_shortest_paths
    }

    fn get_score(&self) -> usize {
        let mut min_score = usize::MAX;
        for d in Direction::all_directions() {
            if let Some(distance) = self.grid.distances.get(&(self.grid.end, d)) {
                min_score = min_score.min(*distance);
            }
        }
        min_score
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        let mut start = Point { x: 0, y: 0 };
        let mut end = Point { x: 0, y: 0 };
        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let el = Field::from_str(&c.to_string())?;
                match el {
                    Field::Start => start = Point { x, y },
                    Field::End => end = Point { x, y },
                    _ => {}
                }
                row.push(if el == Field::Wall {
                    Field::Wall
                } else {
                    Field::Empty
                });
            }
            grid.push(row);
        }

        let mut distances = HashMap::new();
        let mut queue = PriorityQueue::new();
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                let el = grid[y][x];
                if el == Field::Empty {
                    for d in Direction::all_directions() {
                        queue.push((Point { x, y }, d), Reverse(usize::MAX));
                    }
                }
            }
        }
        distances.insert((start, Direction::Right), 0);
        queue.push((start, Direction::Right), Reverse(0));

        Ok(Grid {
            grid: grid,
            distances,
            previous: HashMap::new(),
            queue,
            start,
            end,
        })
    }
}

fn part1(input: &str) -> usize {
    Grid::from_str(input).unwrap().calculate_paths().get_score()
}

fn part2(input: &str) -> usize {
    Grid::from_str(input)
        .unwrap()
        .calculate_paths()
        .find_shortest_paths()
        .count_fields()
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode24/day16.txt");
    format!(
        "Day 16\tPart 1: {}\t Part 2: {}",
        part1(input),
        part2(input)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
    const INPUT2: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;
    #[test]
    fn day1() {
        assert_eq!(part1(INPUT), 7036);
        assert_eq!(part1(INPUT2), 11048);
    }
    #[test]
    fn day2() {
        assert_eq!(part2(INPUT), 45);
        assert_eq!(part2(INPUT2), 64);
    }
}
