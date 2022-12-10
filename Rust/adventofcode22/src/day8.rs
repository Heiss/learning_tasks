use crate::day8::Field::Visible;

#[derive(Debug, PartialEq, PartialOrd)]
enum Field {
    NotVisited(u8),
    Visible(u8),
    Hidden(u8),
}

impl Field {
    fn get_value(&self) -> u8 {
        match self {
            Field::NotVisited(value) => *value,
            Field::Visible(value) => *value,
            Field::Hidden(value) => *value,
        }
    }
}

impl TryFrom<char> for Field {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_digit(10) {
            Some(v) => Ok(Field::NotVisited(v as u8)),
            _ => Err("Invalid field value"),
        }
    }
}

type Cells = Vec<Vec<Field>>;

struct Grid {
    cells: Cells,
}

impl Grid {
    fn new(cells: Cells) -> Grid {
        let grid = Grid { cells };
        grid
    }

    fn get_visible_cells(&mut self) -> Vec<&Field> {
        // make the border visible
        let len = self.cells.len();
        for i in 0..len {
            self.cells[0][i] = Visible(self.cells[0][i].get_value());
            self.cells[i][0] = Visible(self.cells[i][0].get_value());
            self.cells[len - 1][i] = Visible(self.cells[len - 1][i].get_value());
            self.cells[i][len - 1] = Visible(self.cells[i][len - 1].get_value());
        }

        for row in 0..len {
            for column in 0..len {
                self.cells[row][column] = match self.cells[row][column] {
                    Field::NotVisited(v) => {
                        if self.cells[row]
                            .iter()
                            .take(column)
                            .map(Field::get_value)
                            .any(|c| c >= v)
                            && self.cells[row]
                                .iter()
                                .skip(column + 1)
                                .map(Field::get_value)
                                .any(|c| c >= v)
                            && self
                                .cells
                                .iter()
                                .map(|row| &row[column])
                                .take(row)
                                .map(Field::get_value)
                                .any(|c| c >= v)
                            && self
                                .cells
                                .iter()
                                .map(|row| &row[column])
                                .skip(row + 1)
                                .map(Field::get_value)
                                .any(|c| c >= v)
                        {
                            Field::Hidden(v)
                        } else {
                            Field::Visible(v)
                        }
                    }
                    _ => continue,
                };
            }
        }

        self.cells
            .iter()
            .flatten()
            .filter(|cell| match cell {
                Field::Visible(_) => true,
                _ => false,
            })
            .collect()
    }

    fn get_scenic_score_list(&self) -> Vec<usize> {
        let mut result = Vec::new();
        let len = self.cells.len();
        for row in 0..len {
            for column in 0..len {
                let current = self.cells[row][column].get_value();
                let mut left = self.cells[row]
                    .iter()
                    .take(column)
                    .rev()
                    .map(Field::get_value)
                    .take_while(|cell| *cell < current)
                    .count();
                let mut right = self.cells[row]
                    .iter()
                    .skip(column + 1)
                    .map(Field::get_value)
                    .take_while(|cell| *cell < current)
                    .count();
                let mut top = self
                    .cells
                    .iter()
                    .map(|row| &row[column])
                    .take(row)
                    .rev()
                    .map(Field::get_value)
                    .take_while(|cell| *cell < current)
                    .count();
                let mut bottom = self
                    .cells
                    .iter()
                    .map(|row| &row[column])
                    .skip(row + 1)
                    .map(Field::get_value)
                    .take_while(|cell| *cell < current)
                    .count();

                if len - 1 - column > right {
                    right += 1;
                }
                if len - 1 - row > bottom {
                    bottom += 1;
                }
                if column > left {
                    left += 1;
                }
                if row > top {
                    top += 1;
                }
                result.push(left * right * top * bottom);
            }
        }

        result
    }
}

impl TryFrom<&str> for Grid {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut cells = Vec::new();
        for line in value.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(Field::try_from(c)?);
            }
            cells.push(row);
        }

        // check if cell is quadratic
        assert!(cells.iter().all(|row| row.len() == cells.len()));
        Ok(Grid::new(cells))
    }
}

fn day8_part1(text: &str) -> u64 {
    Grid::try_from(text)
        .unwrap()
        .get_visible_cells()
        .iter()
        .count() as u64
}

fn day8_part2(text: &str) -> u64 {
    Grid::try_from(text)
        .unwrap()
        .get_scenic_score_list()
        .iter()
        .max()
        .unwrap()
        .clone() as u64
}

pub fn day8(text: &str) {
    println!("day8 part 1: {}", day8_part1(text));
    println!("day8 part 2: {}", day8_part2(text));
}

#[cfg(test)]
mod tests {
    use super::{day8_part1, day8_part2};

    const TEST_INPUT: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn fixed_tests1() {
        assert_eq!(day8_part1(TEST_INPUT), 21);
    }

    #[test]
    fn fixed_tests2() {
        assert_eq!(day8_part2(TEST_INPUT), 8);
    }
}
