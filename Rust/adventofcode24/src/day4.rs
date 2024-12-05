use itertools::Itertools;
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::num::NonZeroUsize;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, Hash)]
enum Char {
    X,
    M,
    A,
    S,
    Dot,
}

impl PartialEq<Char> for Char {
    fn eq(&self, other: &Char) -> bool {
        match (self, other) {
            (Char::Dot, _) | (_, Char::Dot) => true,
            (Char::A, Char::A) | (Char::X, Char::X) | (Char::M, Char::M) | (Char::S, Char::S) => {
                true
            }
            _ => false,
        }
    }
}

impl Char {
    fn from_char(s: char) -> Self {
        match s {
            'X' => Char::X,
            'M' => Char::M,
            'A' => Char::A,
            'S' => Char::S,
            _ => Char::Dot,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Char::X => 'X',
            Char::M => 'M',
            Char::A => 'A',
            Char::S => 'S',
            Char::Dot => '.',
        }
    }
}

#[derive(Clone, Eq, Hash)]
struct Grid {
    values: Vec<Vec<Char>>,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            values: s
                .lines()
                .map(|l| l.chars().map(Char::from_char).collect())
                .collect(),
        })
    }
}

impl PartialEq<Self> for Grid {
    fn eq(&self, other: &Self) -> bool {
        if self.values.len() != other.values.len() || self.values[0].len() != other.values[0].len()
        {
            return false;
        }

        self.values
            .iter()
            .zip(other.values.iter())
            .flat_map(|(l, r)| l.iter().zip(r.iter()))
            .all(|(l, r)| l == r)
    }
}

impl Grid {
    fn flipped_vertical(&self) -> Grid {
        let mut new = self.values.clone();
        let max_column = self.values[0].len() - 1;
        for i in 0..self.values.len() {
            for j in 0..=max_column {
                new[i][max_column.checked_sub(j).expect("Should not happen")] = self.values[i][j];
            }
        }
        Grid { values: new }
    }
    fn flipped_horizontal(&self) -> Grid {
        let mut new = self.values.clone();
        let max_row = self.values.len() - 1;
        for i in 0..=max_row {
            for j in 0..self.values[0].len() {
                new[max_row.checked_sub(i).expect("Should not be here")][j] = self.values[i][j];
            }
        }
        let v = Grid { values: new };
        v
    }
    fn transposed(&self) -> Grid {
        let mut new = self.values.clone();
        for i in 0..self.values.len() {
            for j in 0..self.values[0].len() {
                new[j][i] = self.values[i][j];
            }
        }
        Grid { values: new }
    }

    fn windows(&self, n: usize) -> GridWindow<'_> {
        GridWindow::new(&self, n)
    }

    fn count_xmas(&self) -> usize {
        let mut variants = Vec::new();
        variants.push(Grid::from_str(
            r#"XMAS
....
....
...."#,
        ));
        variants.push(Grid::from_str(
            r#"X...
.M..
..A.
...S"#,
        ));
        let mut variants: Vec<Grid> = variants.into_iter().map(|g| g.unwrap()).collect();
        variants.append(&mut variants.iter().map(Grid::flipped_horizontal).collect());
        variants.append(&mut variants.iter().map(Grid::flipped_vertical).collect());
        variants.append(&mut variants.iter().map(Grid::transposed).collect());
        variants.append(&mut variants.iter().map(Grid::flipped_horizontal).collect());
        variants.append(&mut variants.iter().map(Grid::flipped_vertical).collect());
        variants.append(&mut variants.iter().map(Grid::transposed).collect());
        let variants: Vec<Grid> = variants.into_iter().unique().collect();

        self.windows(4).filter(|g| variants.contains(g)).count()
    }
}

struct GridWindow<'a> {
    v: &'a Grid,
    size: NonZeroUsize,
    current_row: usize,
    current_column: usize,
}

impl<'a> GridWindow<'a> {
    fn new(v: &'a Grid, size: usize) -> Self {
        Self {
            v,
            size: NonZeroUsize::new(size).unwrap(),
            current_row: 0,
            current_column: 0,
        }
    }
}

impl<'a> Iterator for GridWindow<'a> {
    type Item = Grid;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_column < (self.v.values[0].len() - self.size.get()) {
            self.current_column += 1;
        } else {
            self.current_column = 0;
            if self.current_row < (self.v.values.len() - self.size.get()) {
                self.current_row += 1;
            } else {
                return None;
            }
        }
        let max_row = self.current_row + self.size.get();
        let max_column = self.current_column + self.size.get();
        let vs = &self.v.values[self.current_row..max_row].to_vec();

        let mut res = Vec::new();
        for v in vs {
            res.push(v[self.current_column..max_column].to_vec());
        }
        Some(Grid { values: res })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        for line in self.values.iter() {
            for c in line.iter() {
                res.push(c.to_char());
            }
            res.push('\n');
        }
        write!(f, "{}", res)
    }
}

fn part1(input: &str) -> usize {
    Grid::from_str(input).unwrap().count_xmas()
}

fn part2(input: &str) -> usize {
    0
}

pub fn day() -> String {
    let input = include_str!("../input/day4.txt");
    format!("Day 4\tPart 1: {}\t Part 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(part1(INPUT), 18);
    }
    #[test]
    fn day2() {
        const INPUT: &str =
            r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        assert_eq!(part2(INPUT), 48);
    }
}
