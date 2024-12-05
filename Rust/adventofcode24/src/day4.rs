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
            (Char::A, Char::A) | (Char::X, Char::X) | (Char::M, Char::M) | (Char::S, Char::S) => {
                true
            }
            (Char::Dot, _) | (_, Char::Dot) => false,
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

#[derive(PartialEq)]
enum CompareChar {
    Equal,
    Unequal,
    Ignore,
}

impl PartialEq<Self> for Grid {
    fn eq(&self, other: &Self) -> bool {
        if self.values.len() != other.values.len() || self.values[0].len() != other.values[0].len()
        {
            return false;
        }

        let mut iter = self
            .values
            .iter()
            .zip(other.values.iter())
            .flat_map(|(l, r)| l.iter().zip(r.iter()))
            .map(|(l, r)| match (l, r) {
                (Char::Dot, _) | (_, Char::Dot) => CompareChar::Ignore,
                (Char::X, Char::X)
                | (Char::M, Char::M)
                | (Char::A, Char::A)
                | (Char::S, Char::S) => CompareChar::Equal,
                _ => CompareChar::Unequal,
            })
            .filter(|v| v != &CompareChar::Ignore);

        // ugly to check, if you search for XMAS or X-MAS
        let count = if self.values.len() == 7 { 4 } else { 5 };
        if (iter.clone().count() == count) && iter.all(|v| v == CompareChar::Equal) {
            true
        } else {
            false
        }
    }
}

#[derive(Clone)]
struct PaddedGrid {
    grid: Grid,
    padding_size: NonZeroUsize,
}

impl IntoIterator for PaddedGrid {
    type Item = PaddedGrid;
    type IntoIter = IterPaddedGrid;

    fn into_iter(self) -> Self::IntoIter {
        let mut all_xs = Vec::new();
        let mut all_as = Vec::new();
        for (i, vs) in self.grid.values.iter().enumerate() {
            for (j, c) in vs.iter().enumerate() {
                match c {
                    Char::X => all_xs.push((j, i)),
                    Char::A => all_as.push((j, i)),
                    _ => {}
                };
            }
        }
        IterPaddedGrid {
            grid: self,
            xs: all_xs,
            r#as: all_as,
            iter_for: Char::X,
        }
    }
}

impl PaddedGrid {
    fn get_window(&self, n: NonZeroUsize, x: usize, y: usize) -> Grid {
        let min_column = x - n.get();
        let max_column = x + n.get();
        let min_row = y - n.get();
        let max_row = y + n.get();

        let mut new_grid = Vec::new();
        for i in min_row..=max_row {
            new_grid.push(self.grid.values[i][min_column..=max_column].to_vec());
        }
        Grid { values: new_grid }
    }

    fn count_xmas(self) -> usize {
        let mut variants = Vec::new();
        variants.push(Grid::from_str(
            r#".......
.......
.......
...XMAS
.......
.......
......."#,
        ));
        variants.push(Grid::from_str(
            r#".......
.......
.......
...X...
....M..
.....A.
......S"#,
        ));

        let mut variants: Vec<Grid> = variants.into_iter().map(|g| g.unwrap()).collect();
        variants.append(&mut variants.iter().map(Grid::flipped_vertical).collect());
        variants.append(&mut variants.iter().map(Grid::flipped_horizontal).collect());
        variants.append(&mut variants.iter().map(Grid::transposed).collect());
        variants.append(&mut variants.iter().map(Grid::flipped_vertical).collect());
        variants.append(&mut variants.iter().map(Grid::flipped_horizontal).collect());
        let variants: Vec<Grid> = variants.into_iter().unique().collect();

        self.into_iter()
            .map(|g| {
                let c = variants
                    .iter()
                    .filter(|&v| *v == Grid::from(g.clone()))
                    .count();
                c
            })
            .sum()
    }

    fn count_mas(self) -> usize {
        let mut variants = Vec::new();
        variants.push(Grid::from_str(
            r#"M.S
.A.
M.S"#,
        ));

        let mut variants: Vec<Grid> = variants.into_iter().map(|g| g.unwrap()).collect();
        variants.append(&mut variants.iter().map(Grid::flipped_vertical).collect());
        variants.append(&mut variants.iter().map(Grid::flipped_horizontal).collect());
        variants.append(&mut variants.iter().map(Grid::transposed).collect());
        variants.append(&mut variants.iter().map(Grid::flipped_vertical).collect());
        variants.append(&mut variants.iter().map(Grid::flipped_horizontal).collect());
        variants.append(&mut variants.iter().map(Grid::flipped_vertical).collect());
        variants.append(&mut variants.iter().map(Grid::flipped_horizontal).collect());
        variants.append(&mut variants.iter().map(Grid::transposed).collect());
        variants.append(&mut variants.iter().map(Grid::flipped_vertical).collect());
        variants.append(&mut variants.iter().map(Grid::flipped_horizontal).collect());
        let variants: Vec<Grid> = variants.into_iter().unique().collect();

        self.into_iter()
            .set_iter_for(Char::A)
            .map(|g| {
                let c = variants
                    .iter()
                    .filter(|&v| *v == Grid::from(g.clone()))
                    .count();
                c
            })
            .sum()
    }
}

impl From<PaddedGrid> for Grid {
    fn from(value: PaddedGrid) -> Self {
        value.grid
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

    fn _windows(&self, n: usize) -> GridWindow<'_> {
        GridWindow::new(&self, n)
    }

    fn add_padding(mut self, n: NonZeroUsize) -> PaddedGrid {
        let max_row = self.values.len();
        let max_column = self.values[0].len();
        let row_padding: Vec<Char> = (0..max_column + 2 * n.get()).map(|_| Char::Dot).collect();
        let column_padding: Vec<Char> = (0..n.get()).map(|_| Char::Dot).collect();

        for i in 0..max_row {
            let mut new_row = column_padding.clone();
            new_row.append(&mut self.values[i]);
            new_row.append(&mut column_padding.clone());
            self.values[i] = new_row;
        }

        for _ in 0..n.get() {
            self.values.insert(0, row_padding.clone());
            self.values.push(row_padding.clone());
        }

        PaddedGrid {
            grid: self,
            padding_size: n,
        }
    }
}

struct IterPaddedGrid {
    grid: PaddedGrid,
    xs: Vec<(usize, usize)>,
    r#as: Vec<(usize, usize)>,
    iter_for: Char,
}

impl IterPaddedGrid {
    fn set_iter_for(&mut self, c: Char) -> &mut Self {
        self.iter_for = c;
        self
    }

    fn next_x(&mut self) -> Option<PaddedGrid> {
        if self.xs.len() == 0 {
            None
        } else {
            let (x, y) = self.xs.pop().unwrap();
            let grid = self.grid.get_window(NonZeroUsize::new(3).unwrap(), x, y);

            Some(PaddedGrid {
                grid,
                padding_size: self.grid.padding_size,
            })
        }
    }

    fn next_a(&mut self) -> Option<PaddedGrid> {
        if self.r#as.len() == 0 {
            None
        } else {
            let (x, y) = self.r#as.pop().unwrap();
            let grid = self.grid.get_window(NonZeroUsize::new(1).unwrap(), x, y);

            Some(PaddedGrid {
                grid,
                padding_size: self.grid.padding_size,
            })
        }
    }
}

impl Iterator for IterPaddedGrid {
    type Item = PaddedGrid;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter_for {
            Char::X => self.next_x(),
            Char::A => self.next_a(),
            _ => panic!("Not valid Character"),
        }
    }
}

// All code from below is not needed anymore and was written for a previous approach,
// but i liked the exercise, how to implement windows iterator in rust
#[allow(dead_code)]
struct GridWindow<'a> {
    v: &'a Grid,
    size: NonZeroUsize,
    current_row: usize,
    current_column: usize,
}

impl<'a> GridWindow<'a> {
    #[allow(dead_code)]
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
    Grid::from_str(input)
        .unwrap()
        .add_padding(NonZeroUsize::new(3).unwrap())
        .count_xmas()
}

fn part2(input: &str) -> usize {
    Grid::from_str(input)
        .unwrap()
        .add_padding(NonZeroUsize::new(1).unwrap())
        .count_mas()
}

pub fn day() -> String {
    let input = include_str!("../input/day4.txt");
    format!("Day 4\tPart 1: {}\t Part 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

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
    #[test]
    fn day1() {
        assert_eq!(part1(INPUT), 18);
    }
    #[test]
    fn day2() {
        assert_eq!(part2(INPUT), 9);
    }
}
