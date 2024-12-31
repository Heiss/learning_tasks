use itertools::Itertools;
use std::collections::HashMap;
use std::mem;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn distance(&self, p: &Point) -> f64 {
        (((p.x.abs_diff(self.x)).pow(2) + (p.y.abs_diff(self.y)).pow(2)) as f64).sqrt()
    }

    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
}

struct Grid {
    map: HashMap<char, Vec<Point>>,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<char, Vec<Point>> = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.char_indices() {
                map.entry(c)
                    .or_default()
                    .push(Point::new(x as isize, y as isize))
            }
        }

        Ok(Self { map })
    }
}

impl Grid {
    fn clusters(&mut self) -> Clusters {
        let mut clusters = Vec::new();
        for (_, v) in self.map.iter() {
            let mut vs: Vec<Vec<Point>> = v.iter().map(|v| vec![*v]).collect();

            if vs.len() > 1 {
                let mut changed = true;
                while changed {
                    changed = false;

                    for i in 0..vs.len() {
                        if vs[i].is_empty() {
                            continue;
                        }

                        let mut vi = mem::take(&mut vs[i]);
                        for j in i + 1..vs.len() {
                            let mut vj = mem::take(&mut vs[j]);
                            if vi.iter().any(|x| vj.iter().any(|y| x.distance(y) <= 1.0)) {
                                vi.append(&mut vj);
                                changed = true;
                            } else {
                                mem::swap(&mut vs[j], &mut vj);
                            }
                        }
                        mem::swap(&mut vs[i], &mut vi);
                    }
                }
            }

            for v in vs.into_iter().filter(|v| !v.is_empty()) {
                clusters.push(Cluster { ps: v });
            }
        }
        Clusters(clusters)
    }
}

struct Cluster {
    ps: Vec<Point>,
}

impl Cluster {
    fn area(&self) -> usize {
        self.ps.len()
    }

    fn perimeter(&self) -> usize {
        let mut perimeter = 0;

        for p in &self.ps {
            let v = vec![
                self.ps.contains(&Point::new(p.x - 1, p.y)),
                self.ps.contains(&Point::new(p.x, p.y - 1)),
                self.ps.contains(&Point::new(p.x + 1, p.y)),
                self.ps.contains(&Point::new(p.x, p.y + 1)),
            ];
            perimeter += 4 - v.iter().filter(|&i| *i).count();
        }

        perimeter
    }

    fn sides(&self) -> usize {
        let mut edges = 0;

        for p in &self.ps {
            edges += vec![1, -1]
                .into_iter()
                .cartesian_product(vec![1, -1])
                .map(|(x, y)| {
                    let row_neighbor = Point::new(p.x + x, p.y);
                    let col_neighbor = Point::new(p.x, p.y + y);
                    let diag_neighbor = Point::new(p.x + x, p.y + y);

                    let mut res = 0;

                    if !self.ps.contains(&row_neighbor) && !self.ps.contains(&col_neighbor) || self.ps.contains(&row_neighbor)
                        && self.ps.contains(&col_neighbor)
                        && !self.ps.contains(&diag_neighbor)
                    {
                        res += 1;
                    }

                    res
                })
                .sum::<usize>();
        }

        edges
    }

    fn price(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn discount(&self) -> usize {
        self.area() * self.sides()
    }
}

struct Clusters(Vec<Cluster>);

impl Clusters {
    fn price(&self) -> usize {
        self.0.iter().map(Cluster::price).sum()
    }

    fn price_discount(&self) -> usize {
        self.0.iter().map(Cluster::discount).sum()
    }
}

fn part1(input: &str) -> usize {
    Grid::from_str(input).unwrap().clusters().price()
}

fn part2(input: &str) -> usize {
    Grid::from_str(input).unwrap().clusters().price_discount()
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode24/day12.txt");
    format!(
        "Day 12\tPart 1: {}\t Part 2: {}",
        part1(input),
        part2(input)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
    #[test]
    fn day1() {
        assert_eq!(
            part1(
                r#"AAAA
BBCD
BBCC
EEEC"#
            ),
            140
        );
        assert_eq!(
            part1(
                r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#
            ),
            772
        );
        assert_eq!(part1(INPUT), 1930);
    }
    #[test]
    fn day2() {
        assert_eq!(
            part2(
                r#"AAAA
BBCD
BBCC
EEEC"#
            ),
            80
        );
        assert_eq!(
            part2(
                r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#
            ),
            436
        );
        assert_eq!(part2(INPUT), 1206);
    }
}
