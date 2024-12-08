use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Antenna {
    x: isize,
    y: isize,
}

impl Antenna {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }

    fn get_antinodes(&self, other: &Self, dimension: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        if self.x == other.x && self.y == other.y || self.x > other.x && self.y > other.y {
            return res;
        }

        let dy = other.y - self.y;
        let dx = other.x - self.x;

        if let (Some(x), Some(y)) = (self.x.checked_sub(dx), self.y.checked_sub(dy)) {
            if (x as usize) < dimension.0 && (y as usize) < dimension.1 {
                res.push((x as usize, y as usize))
            };
        }

        let p = ((other.x + dx) as usize, (other.y + dy) as usize);
        if p.0 < dimension.0 && p.1 < dimension.1 {
            res.push(p);
        }

        res
    }

    fn get_antinodes_lines(&self, other: &Self, dimension: &(usize, usize)) -> Vec<(usize, usize)> {
        let dy = other.y - self.y;
        let dx = other.x - self.x;

        /*
        //I think, it should work with simple linear algebraic functions, but somehow I get to few points.
        //But this approach does work on test input, but not on larger one.
        //So I need to write it out very specific... damn

        let m = dy / dx;
        let n = other.y - m * other.x;

        (0..dimension.0)
            .map(|v| {
                let x = v as f64;
                (v, m * x + n)
            })
            .filter(|(_, y)| y.fract() == 0.0 && *y >= usize::MIN as f64 && *y < dimension.1 as f64)
            .map(|(x, y)| (x, y as usize))
            .collect()
         */

        let mut res = Vec::new();
        let mut k = 0;
        loop {
            let (an_x, an_y) = (self.x - k * dx, self.y - k * dy);
            if an_x < 0 || an_y < 0 || an_x >= dimension.0 as isize || an_y >= dimension.1 as isize
            {
                break;
            }
            res.push((an_x as usize, an_y as usize));
            k += 1;
        }
        let mut k = 0;
        loop {
            let (an_x, an_y) = (other.x + k * dx, other.y + k * dy);
            if an_x < 0 || an_y < 0 || an_x >= dimension.0 as isize || an_y >= dimension.1 as isize
            {
                break;
            }
            res.push((an_x as usize, an_y as usize));
            k += 1;
        }

        res
    }

    fn from_str(s: &str) -> (HashMap<char, Vec<Antenna>>, (usize, usize)) {
        let mut map: HashMap<char, Vec<Antenna>> = HashMap::new();
        let dim_x = s.lines().next().unwrap().chars().count();
        let dim_y = s.lines().count();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.char_indices() {
                if c != '.' {
                    map.entry(c).or_default().push(Self::new(x, y));
                }
            }
        }
        (map, (dim_x, dim_y))
    }
}

fn part1(input: &str) -> usize {
    let mut set = HashSet::new();
    let (map, dim) = Antenna::from_str(input);
    for (_, antennas) in map {
        for as1 in &antennas {
            for as2 in &antennas {
                let antinodes = as1.get_antinodes(as2, &dim);
                for a in antinodes {
                    set.insert(a);
                }
            }
        }
    }
    set.len()
}

fn part2(_input: &str) -> usize {
    let mut set = HashSet::new();
    let (map, dim) = Antenna::from_str(_input);
    for (_, antennas) in &map {
        for (i, as1) in antennas.iter().enumerate() {
            for as2 in antennas.iter().skip(i + 1) {
                let antinodes = as1.get_antinodes_lines(as2, &dim);
                for a in antinodes {
                    set.insert(a);
                }
            }
        }
    }
    set.len()
}

pub fn day() -> String {
    let input = include_str!("../input/day8.txt");
    format!("Day 8\tPart 1: {}\t Part 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
    #[test]
    fn day1() {
        assert_eq!(
            part1(
                r#"..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
.........."#
            ),
            2
        );
        assert_eq!(
            part1(
                r#"..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
.........."#
            ),
            4
        );
        assert_eq!(part1(INPUT), 14);
    }
    #[test]
    fn day2() {
        assert_eq!(
            part2(
                r#"T.........
...T......
.T........
...t......
..........
..........
..........
..........
..........
.........."#
            ),
            9
        );
        assert_eq!(
            part2(
                r#"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."#
            ),
            9
        );
        assert_eq!(part2(INPUT), 34);
    }
}
