use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Field {
    Empty,
    CubeRock,
    RoundedRock,
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Field::Empty),
            "#" => Ok(Field::CubeRock),
            "O" => Ok(Field::RoundedRock),
            _ => panic!("Invalid field: {}", s),
        }
    }
}

struct Grid {
    fields: Vec<Vec<Field>>,
}

impl Grid {
    /// Rotates the grid clockwise by 90 degrees.
    fn rotate(&mut self) {
        let mut new_fields = Vec::new();
        for x in 0..self.fields[0].len() {
            let mut new_row = Vec::new();
            for y in (0..self.fields.len()).rev() {
                new_row.push(self.fields[y][x]);
            }
            new_fields.push(new_row);
        }
        self.fields = new_fields;
    }

    fn find_resting_position_north(&self, x: usize, mut y: usize) -> (usize, usize) {
        while y > 0 {
            match self.fields[y - 1][x] {
                Field::Empty => y -= 1,
                Field::CubeRock | Field::RoundedRock => return (x, y),
            }
        }
        (x, y)
    }

    /// Tilts the grid to the north, so all rounded rocks rolls up until there is the end of the plattform, a cube rock or another rounded cube.
    fn tilt_north(&mut self) {
        for y in 1..self.fields.len() {
            for x in 0..self.fields[0].len() {
                if let Field::RoundedRock = self.fields[y][x] {
                    let (new_x, new_y) = self.find_resting_position_north(x, y);
                    if new_y != y {
                        self.fields[new_y][new_x] = Field::RoundedRock;
                        self.fields[y][x] = Field::Empty;
                    }
                }
            }
        }
    }

    /// Calculates the total load of the grid.
    /// The amount of load caused by a single rounded rock (O) is equal to the number of rows from the rock to the south edge of the platform, including the row the rock is on.
    fn get_total_load(&self) -> usize {
        let max_load = self.fields.len();
        let mut load = 0;
        for y in 0..self.fields.len() {
            for x in 0..self.fields[0].len() {
                if let Field::RoundedRock = self.fields[y][x] {
                    load += max_load - y;
                }
            }
        }
        load
    }

    /// Tilt and rotate the grid 4 times.
    fn cycle(&mut self) {
        for _ in 0..4 {
            self.tilt_north();
            self.rotate();
        }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = Vec::new();

        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_string().parse()?);
            }
            fields.push(row);
        }

        Ok(Grid { fields })
    }
}

fn part1(input: &str) -> usize {
    let mut grid = input.parse::<Grid>().unwrap();
    grid.tilt_north();
    grid.get_total_load()
}

fn part2(input: &str) -> usize {
    let mut grid = input.parse::<Grid>().unwrap();
    let mut cache: HashMap<Vec<Vec<Field>>, usize> = HashMap::new();

    let mut cycle = 0;
    let goal = 1_000_000_000;
    let mut cycle_length = None;

    while cycle < goal {
        grid.cycle();
        cycle += 1;

        if cycle_length.is_none() && cache.contains_key(&grid.fields) {
            cycle_length = Some(cycle - cache[&grid.fields]);
            cycle += cycle_length.unwrap() * ((goal - cycle) / cycle_length.unwrap());
        } else {
            cache.insert(grid.fields.clone(), cycle);
        }
    }

    grid.get_total_load()
}

pub fn day() {
    let input = include_str!("../input/day14.txt");
    print!("day 14\t");
    print!("part 1: {}\t", part1(input));
    print!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 136);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 64);
    }
}
