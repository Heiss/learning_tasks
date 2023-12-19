use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Field {
    DigOut,
    Dirt,
}

#[derive(Debug, PartialEq, Clone)]
struct Plan {
    dir: Direction,
    steps: usize,
}

impl Plan {
    fn new(dir: Direction, steps: usize) -> Self {
        Plan { dir, steps }
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
    fn from_num(p0: &str) -> Self {
        match p0 {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => panic!("Invalid direction {}", p0),
        }
    }
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

    fn add(&self, p: Point) -> Point {
        Point::new(self.x + p.x, self.y + p.y)
    }

    fn add_plan(&self, plan: &Plan) -> Point {
        let dir = plan.dir.to_point();
        Point::new(
            self.x + dir.x * plan.steps as isize,
            self.y + dir.y * plan.steps as isize,
        )
    }
}

#[derive(PartialEq, Clone)]
struct Grid {
    plans: Vec<Plan>,
}

#[allow(dead_code)]
fn print_fields(fields: &Vec<Vec<Field>>) {
    for row in fields {
        for field in row {
            match field {
                Field::Dirt => print!("."),
                Field::DigOut => print!("#"),
            }
        }
        println!();
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut plans = Vec::new();
        for line in s.lines() {
            let plan = line.split_whitespace().collect::<Vec<_>>();
            let dir = Direction::from_str(plan[0]);
            let steps = plan[1].parse().unwrap();
            plans.push(Plan::new(dir, steps));
        }

        Ok(Grid { plans })
    }
}

impl Grid {
    fn get_fields(&self) -> Vec<Vec<Field>> {
        let mut pos = Point::new(0, 0);

        let mut possitions = Vec::new();
        for plan in &self.plans {
            for _ in 0..plan.steps {
                pos = pos.add(plan.dir.to_point());
                possitions.push(pos.clone());
            }
        }

        let find_min_x = possitions.iter().map(|p| p.x).min().unwrap();
        let find_max_x = possitions.iter().map(|p| p.x).max().unwrap();
        let find_min_y = possitions.iter().map(|p| p.y).min().unwrap();
        let find_max_y = possitions.iter().map(|p| p.y).max().unwrap();

        let mut fields = vec![
            vec![Field::Dirt; (find_max_x - find_min_x) as usize + 1];
            (find_max_y - find_min_y) as usize + 1
        ];

        for pos in possitions {
            let x = pos.x - find_min_x;
            let y = pos.y - find_min_y;
            fields[y as usize][x as usize] = Field::DigOut;
        }

        fields
    }

    fn from_hex_str(s: &str) -> Self {
        use regex::Regex;

        let mut plans = Vec::new();
        let re = Regex::new(r"^[URDL] \d+ \(#([a-f0-9]{5})(\d)\)$").unwrap();

        for line in s.lines() {
            let cap = re.captures_iter(line).next();

            let cap = if cap.is_none() {
                panic!("Invalid input {}", line);
            } else {
                cap.unwrap()
            };

            let steps = usize::from_str_radix(&cap[1], 16).unwrap();
            let dir = Direction::from_num(&cap[2]);

            plans.push(Plan::new(dir, steps));
        }

        Grid { plans }
    }

    /// Count the number of inner dirt fields.
    /// Here it is done through a pixel by pixel search.
    ///
    /// It goes from left to right, top to bottom.
    /// If it encounter a dig out field, it looksup the field up and down, if there is a digout field, too and save the direction.
    /// It goes to the next field and checks for a digout field. If up and down are digout fields too, it checks if the directions are the same.
    /// If the direction changes, all next dirt fields in row are inside and should be counted.
    fn count_inner_dirt_pixel(&self) -> usize {
        let mut grid_fields = self.get_fields();
        let fields = grid_fields.clone();

        for y in 1..fields.len() - 1 {
            let mut inner = false;
            let mut dir = None;
            for x in 0..fields[y].len() {
                match fields[y][x] {
                    Field::Dirt => {
                        if inner {
                            grid_fields[y][x] = Field::DigOut;
                        }
                        dir = None;
                    }
                    Field::DigOut => {
                        let up = fields.get(y - 1).map(|row| row[x]);
                        let down = fields.get(y + 1).map(|row| row[x]);

                        match (up, down) {
                            (Some(Field::DigOut), Some(Field::DigOut)) => inner = !inner,
                            (Some(Field::DigOut), _) => {
                                if dir == Some(Direction::Down) {
                                    inner = !inner;
                                } else {
                                    dir = Some(Direction::Up);
                                }
                            }
                            (_, Some(Field::DigOut)) => {
                                if dir == Some(Direction::Up) {
                                    inner = !inner;
                                } else {
                                    dir = Some(Direction::Down);
                                }
                            }
                            (None, None)
                            | (None, Some(Field::Dirt))
                            | (Some(Field::Dirt), Some(Field::Dirt))
                            | (Some(Field::Dirt), None) => {}
                        }
                    }
                }
            }
        }
        //print_fields(&grid_fields);

        grid_fields
            .iter()
            .flat_map(|row| row.iter())
            .filter(|f| match f {
                Field::DigOut => true,
                _ => false,
            })
            .count()
    }

    /// Count the number of inner dirt fields.
    /// Here it is done through a shoelace algorithm and respects picks theorem.
    ///
    /// https://www.themathdoctors.org/polygon-coordinates-and-areas/
    fn count_inner_dirt_with_shoelace(&self) -> usize {
        let plans = self.plans.clone();

        let mut previous_point;
        let mut current_point = Point { x: 0, y: 0 };

        let mut count_points_on_boundary = 0;
        let mut area_interior = 0;

        for plan in &plans {
            previous_point = current_point;
            current_point = current_point.add_plan(plan);

            count_points_on_boundary += plan.steps;
            area_interior +=
                previous_point.x * current_point.y - current_point.x * previous_point.y;
        }

        count_points_on_boundary / 2 + area_interior.abs() as usize / 2 + 1
    }
}

fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    grid.count_inner_dirt_pixel()
}

fn part2(input: &str) -> usize {
    let grid = Grid::from_hex_str(input);
    grid.count_inner_dirt_with_shoelace()
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
        assert_eq!(part2(INPUT), 952408144115);
    }
}
