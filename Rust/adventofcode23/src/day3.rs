struct Matrix {
    row: Vec<Vec<char>>,
}

impl From<&str> for Matrix {
    fn from(input: &str) -> Self {
        let data = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Self { row: data }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

type Points = Vec<Point>;

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl Matrix {
    fn find_all_symbols(&self) -> Vec<(char, Point)> {
        let mut result = Vec::new();
        for (y, line) in self.row.iter().enumerate() {
            for (x, &symbol) in line.iter().enumerate() {
                if symbol != '.' && !symbol.is_digit(10) {
                    result.push((symbol, Point::new(x, y)));
                }
            }
        }
        result
    }

    fn get_gears(&self) -> Vec<Point> {
        self.find_all_symbols()
            .iter()
            .filter(|(symbol, _point)| *symbol == '*')
            .map(|(_symbol, point)| *point)
            .collect::<Vec<Point>>()
    }

    fn get_gear_ratio(&self) -> usize {
        self.get_gears()
            .iter()
            .map(|point| self.get_adjacent_numbers(point))
            .filter(|vec| vec.len() == 2)
            .map(|vec| vec[0] * vec[1])
            .sum()
    }

    fn get_symbol(&self, point: &Point) -> Option<char> {
        if point.y >= self.row.len() || point.x >= self.row[point.y].len() {
            return None;
        }
        Some(self.row[point.y][point.x])
    }

    fn get_complete_number(&self, point: &Point) -> Option<(usize, Points)> {
        let symbol = self.get_symbol(&point)?;
        let mut number: Vec<char> = vec![symbol];
        let mut points: Points = vec![point.clone()];

        let mut left = point.x - 1;
        let mut right = point.x + 1;
        let row = point.y;

        while let Some(symbol) = self.get_symbol(&Point::new(left, row)) {
            if !symbol.is_digit(10) {
                break;
            }
            number.insert(0, symbol);
            points.push(Point::new(left, row));

            if left == 0 {
                break;
            }
            left -= 1;
        }

        while let Some(symbol) = self.get_symbol(&Point::new(right, row)) {
            if !symbol.is_digit(10) || right == self.row[row].len() {
                break;
            }
            number.push(symbol);
            points.push(Point::new(right, row));
            right += 1;
        }

        if number.is_empty() {
            return None;
        }

        let number = number.iter().collect::<String>().parse::<usize>().unwrap();
        Some((number, points))
    }

    fn get_adjacent_numbers(&self, point: &Point) -> Vec<usize> {
        let mut seen_points: Vec<Point> = Vec::new();
        let mut vec = Vec::new();
        for point in self.get_adjacent_numberpoints(point) {
            if let Some(number) = self.get_complete_number(&point) {
                if number.1.iter().any(|p| seen_points.contains(p)) {
                    continue;
                }
                vec.push(number.0);
                seen_points.extend(number.1);
            }
        }
        vec
    }

    fn get_adjacent_numberpoints(&self, point: &Point) -> Vec<Point> {
        let mut vec = Vec::new();
        let x = point.x;
        let y = point.y;
        let watched_fields = vec![
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];
        let len_row = self.row.len();
        let len_col = self.row[0].len();

        let watched_fields = watched_fields
            .into_iter()
            .filter(|(x, y)| *x < len_col && *y < len_row)
            .map(|(x, y)| (x, y).into())
            .collect::<Vec<Point>>();

        for point in watched_fields {
            if let Some(symbol) = self.get_symbol(&point) {
                if symbol.is_digit(10) {
                    vec.push(point);
                }
            }
        }

        vec
    }

    fn find_all_numbers_with_adjacent_symbols(&self) -> Vec<usize> {
        let mut vec = Vec::new();
        for (_symbol, point) in self.find_all_symbols() {
            for number in self.get_adjacent_numbers(&point) {
                vec.push(number);
            }
        }
        vec
    }
}

fn part1(input: &str) -> usize {
    let matrix = Matrix::from(input);
    matrix.find_all_numbers_with_adjacent_symbols().iter().sum()
}

fn part2(input: &str) -> usize {
    let matrix = Matrix::from(input);
    matrix.get_gear_ratio()
}

pub fn day() {
    let input = include_str!("../input/day3.txt");
    print!("Day 3\t");
    print!("Part 1: {}\t", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 4361);
    }

    #[test]
    fn test_test2() {
        assert_eq!(part2(INPUT), 467835);
    }
}
