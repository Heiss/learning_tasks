use std::fmt::{Debug, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FieldState {
    Ash,
    Rock,
}

impl FromStr for FieldState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(FieldState::Rock),
            "." => Ok(FieldState::Ash),
            _ => panic!("Unknown field state: {}", s)
        }
    }
}

struct MapSlice {
    fields: Vec<Vec<FieldState>>,
}

impl Debug for MapSlice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.fields {
            for field in row {
                match field {
                    FieldState::Ash => write!(f, ".")?,
                    FieldState::Rock => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl MapSlice {
    fn revert_row(&mut self) {
        self.fields.reverse();
    }
}

struct MirrorMap {
    fields: Vec<Vec<FieldState>>,
}

impl Debug for MirrorMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        MapSlice { fields: self.fields.clone() }.fmt(f)
    }
}

impl MirrorMap {
    fn make_row_slice(&self, row: usize) -> (MapSlice, MapSlice) {
        let up = self.fields[..row].to_vec();
        let down = self.fields[row..].to_vec();
        let mut up2 = MapSlice { fields: up };
        up2.revert_row();
        (up2, MapSlice { fields: down })
    }

    fn transpose(&mut self) {
        let mut new_fields = Vec::new();
        for column in 0..self.fields[0].len() {
            let mut new_row = Vec::new();
            for row in &self.fields {
                new_row.push(row[column]);
            }
            new_fields.push(new_row);
        }
        self.fields = new_fields;
    }

    fn check_smudge_mirror(&self, row: usize, smudges: usize) -> bool {
        let (up, down) = self.make_row_slice(row);

        let row_min = std::cmp::min(up.fields.len(), down.fields.len());
        let column_min = std::cmp::min(up.fields[0].len(), down.fields[0].len());

        let mut unequal_fields = 0;
        for row in 0..row_min {
            for column in 0..column_min {
                if up.fields[row][column] != down.fields[row][column] {
                    unequal_fields += 1;
                }
            }
        }
        unequal_fields == smudges
    }


    fn find_smudge_reflection(&mut self, smudges: usize) -> (Option<Row>, Option<Column>) {
        let mut row_index = None;
        for row in 1..self.fields.len() {
            if self.check_smudge_mirror(row, smudges) {
                row_index = Some(row);
                break;
            }
        }

        self.transpose();

        let mut column_index = None;
        for column in 1..self.fields.len() {
            if self.check_smudge_mirror(column, smudges) {
                column_index = Some(column);
                break;
            }
        }

        if let (Some(l), Some(r)) = (row_index, column_index) {
            if l > r {
                row_index = None;
            } else {
                column_index = None;
            }
        }

        (row_index, column_index)
    }
}

type Row = usize;
type Column = usize;

impl FromStr for MirrorMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: Vec<Vec<FieldState>> = Vec::new();
        for line in s.lines() {
            let mut columns = Vec::new();
            for field in line.chars() {
                columns.push(field.to_string().parse().unwrap());
            }
            rows.push(columns);
        }

        Ok(MirrorMap {
            fields: rows
        })
    }
}

struct MirrorMaps {
    maps: Vec<MirrorMap>,
}

impl FromStr for MirrorMaps {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maps = Vec::new();
        for map in s.split("\n\n") {
            maps.push(map.parse().unwrap());
        }

        Ok(MirrorMaps { maps })
    }
}

impl MirrorMaps {
    fn count_smudge_reflections(&mut self, smudges: usize) -> usize {
        self.maps.iter_mut().filter_map(|m| match m.find_smudge_reflection(smudges) {
            (Some(l), Some(r)) => Some(l * 100 + r),
            (Some(v), None) => Some(v * 100),
            (None, Some(v)) => Some(v),
            _ => None
        }).sum()
    }
}

fn part1(input: &str) -> usize {
    let mut maps: MirrorMaps = input.parse().unwrap();
    maps.count_smudge_reflections(0)
}

fn part2(input: &str) -> usize {
    let mut maps: MirrorMaps = input.parse().unwrap();
    maps.count_smudge_reflections(1)
}

pub fn day() {
    let input = include_str!("../input/day13.txt");
    print!("day 13\t");
    print!("part 1: {}\t", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 400);
    }
}
