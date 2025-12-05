use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Tile {
    Roll,
    Empty,
}

#[derive(Debug)]
struct Hall {
    matrix: Vec<Vec<Tile>>,
}

impl FromStr for Hall {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Vec::new();
        for l in s.lines() {
            let mut row = Vec::new();
            for c in l.chars() {
                if c == '@' {
                    row.push(Tile::Roll);
                } else {
                    row.push(Tile::Empty);
                }
            }
            result.push(row);
        }
        let v = Hall { matrix: result };
        Ok(v)
    }
}

impl Hall {
    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        if x >= self.matrix[0].len() || y >= self.matrix.len() {
            return None;
        }

        Some(&self.matrix[y][x])
    }

    fn get_adjacent_tiles(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for dx in -1isize..=1 {
            for dy in -1isize..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let (x, y) = (x as isize + dx, y as isize + dy);

                if x < 0 || y < 0 {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);

                if let Some(_) = self.get(x, y) {
                    result.push((x, y));
                }
            }
        }
        result
    }

    fn get_accessable_rolls(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for y in 0..self.matrix.len() {
            for x in 0..self.matrix[0].len() {
                if let Some(Tile::Empty) = self.get(x, y) {
                    continue;
                }

                if self
                    .get_adjacent_tiles(x, y)
                    .iter()
                    .filter(|&&v| self.get(v.0, v.1).unwrap() == &Tile::Roll)
                    .count()
                    < 4
                {
                    result.push((x, y));
                }
            }
        }
        result
    }

    fn remove_accessable_rolls(&mut self) -> usize {
        if self.matrix.is_empty() {
            return 0;
        }
        let height = self.matrix.len();
        let width = self.matrix[0].len();
        let mut neighbor_counts = vec![vec![0; width]; height];
        let mut queue = Vec::new();

        // 1. Initiale Zählung der Nachbarn und Befüllen der Queue
        for y in 0..height {
            for x in 0..width {
                if self.matrix[y][x] == Tile::Roll {
                    let count = self
                        .get_adjacent_tiles(x, y)
                        .iter()
                        .filter(|&&(nx, ny)| self.matrix[ny][nx] == Tile::Roll)
                        .count();

                    neighbor_counts[y][x] = count;
                    if count < 4 {
                        queue.push((x, y));
                    }
                }
            }
        }

        let mut counter = 0;
        // 2. Abarbeiten der Queue (Domino-Effekt)
        while let Some((x, y)) = queue.pop() {
            // Falls bereits entfernt (könnte theoretisch passieren, wenn Logik nicht strikt,
            // aber hier zur Sicherheit), überspringen.
            if self.matrix[y][x] == Tile::Empty {
                continue;
            }

            self.matrix[y][x] = Tile::Empty;
            counter += 1;

            // Nachbarn aktualisieren
            for (nx, ny) in self.get_adjacent_tiles(x, y) {
                if self.matrix[ny][nx] == Tile::Roll {
                    if neighbor_counts[ny][nx] > 0 {
                        neighbor_counts[ny][nx] -= 1;
                    }
                    // Wenn der Nachbar durch das Entfernen gerade instabil geworden ist (4 -> 3)
                    if neighbor_counts[ny][nx] == 3 {
                        queue.push((nx, ny));
                    }
                }
            }
        }
        counter
    }
}

fn part1(input: &str) -> usize {
    Hall::from_str(input).unwrap().get_accessable_rolls().len()
}

fn part2(input: &str) -> usize {
    Hall::from_str(input).unwrap().remove_accessable_rolls()
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode25/day4.txt");
    format!("Day 4\tPart 1: {}\t Part 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn day1() {
        assert_eq!(part1(INPUT), 13);
    }
    #[test]
    fn day2() {
        assert_eq!(part2(INPUT), 43);
    }
}
