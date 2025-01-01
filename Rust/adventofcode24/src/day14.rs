use std::collections::HashMap;
use std::ops::AddAssign;
use std::str::FromStr;

#[cfg(test)]
static HEIGHT_LIMIT: isize = 7;
#[cfg(test)]
static WIDTH_LIMIT: isize = 11;
#[cfg(not(test))]
static HEIGHT_LIMIT: isize = 103;
#[cfg(not(test))]
static WIDTH_LIMIT: isize = 101;

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Robot {
    starting_position: Point,
    velocity: Point,
}

impl Robot {
    fn position_at(&self, seconds: usize) -> Point {
        Point {
            x: (self.starting_position.x + (self.velocity.x * seconds as isize))
                .rem_euclid(WIDTH_LIMIT),
            y: (self.starting_position.y + (self.velocity.y * seconds as isize))
                .rem_euclid(HEIGHT_LIMIT),
        }
    }
}

struct Grid {
    robots: Vec<Robot>,
    seconds: usize,
}

impl Grid {
    fn safety_factor(&self) -> usize {
        self.safety_factor_at_seconds(self.seconds)
    }

    fn safety_factor_at_seconds(&self, seconds: usize) -> usize {
        let mut quadrants_factors: HashMap<(usize, usize), usize> = HashMap::new();

        for robot in &self.robots {
            let halved_width = WIDTH_LIMIT / 2;
            let halved_height = HEIGHT_LIMIT / 2;
            let position = robot.position_at(seconds);

            if position.x == halved_width || position.y == halved_height {
                continue;
            }

            let quadrant = (
                if position.x < halved_width { 0 } else { 1 },
                if position.y < halved_height { 0 } else { 1 },
            );
            quadrants_factors.entry(quadrant).or_default().add_assign(1);
        }

        quadrants_factors.values().product()
    }

    fn is_tree_at_seconds(&self, seconds: usize) -> bool {
        let grid = self.get_grid_at_seconds(seconds);

        for y in 0..HEIGHT_LIMIT as usize {
            let mut count = 0;
            for x in 0..WIDTH_LIMIT as usize {
                if grid[y][x] > 0 {
                    count += 1;
                    if count >= 10 {
                        return true;
                    }
                } else {
                    count = 0;
                }
            }
        }

        for x in 0..WIDTH_LIMIT as usize {
            let mut count = 0;
            for y in 0..HEIGHT_LIMIT as usize {
                if grid[y][x] > 0 {
                    count += 1;
                    if count >= 10 {
                        return true;
                    }
                } else {
                    count = 0;
                }
            }
        }

        false
    }

    fn get_grid_at_seconds(&self, seconds: usize) -> Vec<Vec<usize>> {
        let mut grid = vec![vec![0; WIDTH_LIMIT as usize]; HEIGHT_LIMIT as usize];

        for robot in &self.robots {
            let position = robot.position_at(seconds);
            grid[position.y as usize][position.x as usize] += 1;
        }

        grid
    }

    fn find_first_tree(&self) -> usize {
        let mut seconds = 0;

        while !self.is_tree_at_seconds(seconds) {
            seconds += 1;
        }

        seconds
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut robots = Vec::new();

        for line in s.lines() {
            let mut numbers = Vec::new();
            let mut current_number = String::new();

            for c in line.chars() {
                if c.is_digit(10) || c == '-' {
                    current_number.push(c);
                } else if !current_number.is_empty() {
                    numbers.push(current_number.parse().unwrap());
                    current_number.clear();
                }
            }

            if !current_number.is_empty() {
                numbers.push(current_number.parse().unwrap());
            }

            robots.push(Robot {
                starting_position: Point {
                    x: numbers.remove(0),
                    y: numbers.remove(0),
                },
                velocity: Point {
                    x: numbers.remove(0),
                    y: numbers.remove(0),
                },
            });
        }

        Ok(Self {
            robots,
            seconds: 100,
        })
    }
}

fn part1(input: &str) -> usize {
    Grid::from_str(input).unwrap().safety_factor()
}

fn part2(input: &str) -> usize {
    Grid::from_str(input).unwrap().find_first_tree()
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode24/day14.txt");
    format!(
        "Day 14\tPart 1: {}\t Part 2: {}",
        part1(input),
        part2(input)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;
    #[test]
    fn day1() {
        assert_eq!(part1(r#"p=2,4 v=2,-3"#), 1);
        assert_eq!(part1(INPUT), 12);
    }
    #[test]
    fn day2() {
        //        assert_eq!(part2(INPUT), 1206);
    }
}
