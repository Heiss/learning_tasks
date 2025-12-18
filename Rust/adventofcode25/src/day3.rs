use std::str::FromStr;

struct Battery(usize);
struct Bank {
    batteries: Vec<Battery>,
}

impl FromStr for Bank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Bank {
            batteries: s
                .chars()
                .map(|v| v.to_digit(10).unwrap())
                .map(|v| Battery(v as usize))
                .collect(),
        })
    }
}

impl Bank {
    fn find_highest_battery_value(&self) -> usize {
        let mut vs = Vec::new();

        for i in 0..self.batteries.len() {
            for j in i + 1..self.batteries.len() {
                vs.push(self.batteries[i].0 * 10 + self.batteries[j].0);
            }
        }

        *vs.iter().max().unwrap()
    }

    // Wir entfernen das Argument 'combinations' und implementieren die Greedy-Suche
    fn find_highest_battery_value_with_12_batteries(&self) -> usize {
        let needed = 12;
        let total_len = self.batteries.len();

        if total_len < needed {
            return 0;
        }

        let mut current_idx = 0;
        let mut result_string = String::with_capacity(needed);

        for k in 0..needed {
            let remaining_needed = needed - 1 - k;
            let search_limit = total_len - remaining_needed;

            let mut best_val = 0;
            let mut best_idx_offset = 0;
            let mut found = false;

            for (offset, i) in (current_idx..search_limit).enumerate() {
                let val = self.batteries[i].0;

                if !found || val > best_val {
                    best_val = val;
                    best_idx_offset = offset;
                    found = true;

                    if best_val == 9 {
                        break;
                    }
                }
            }

            result_string.push_str(&best_val.to_string());

            current_idx += best_idx_offset + 1;
        }

        result_string.parse::<usize>().unwrap_or(0)
    }
}

struct Banks(Vec<Bank>);

impl FromStr for Banks {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Banks(
            s.lines().map(|v| Bank::from_str(v).unwrap()).collect(),
        ))
    }
}

pub fn _generate_combinations_with_12_ones(length: usize) -> Vec<String> {
    // Sicherheitsprüfung: Länge muss mindestens 12 sein
    if length < 12 {
        return Vec::new();
    }

    let mut results = Vec::new();
    let mut current_buffer = String::with_capacity(length);

    // Start der rekursiven Generierung
    _backtrack(length, 12, &mut current_buffer, &mut results);

    results
}

fn _backtrack(
    remaining_len: usize,
    ones_needed: usize,
    buffer: &mut String,
    results: &mut Vec<String>,
) {
    if ones_needed > remaining_len {
        return;
    }

    if ones_needed == 0 {
        let start_len = buffer.len();
        for _ in 0..remaining_len {
            buffer.push('0');
        }
        results.push(buffer.clone());
        buffer.truncate(start_len);
        return;
    }

    if ones_needed == remaining_len {
        let start_len = buffer.len();
        for _ in 0..remaining_len {
            buffer.push('1');
        }
        results.push(buffer.clone());
        buffer.truncate(start_len);
        return;
    }

    buffer.push('0');
    _backtrack(remaining_len - 1, ones_needed, buffer, results);
    buffer.pop();

    buffer.push('1');
    _backtrack(remaining_len - 1, ones_needed - 1, buffer, results);
    buffer.pop();
}

impl Banks {
    fn total_output(&self) -> usize {
        self.0.iter().map(|v| v.find_highest_battery_value()).sum()
    }

    fn total_output_with_12_batteries(&self) -> usize {
        // Keine Vorberechnung von Kombinationen mehr nötig!
        self.0
            .iter()
            .map(|v| v.find_highest_battery_value_with_12_batteries())
            .sum()
    }
}

fn part1(input: &str) -> usize {
    Banks::from_str(input).unwrap().total_output()
}

fn part2(input: &str) -> usize {
    Banks::from_str(input)
        .unwrap()
        .total_output_with_12_batteries()
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode25/day3.txt");
    format!("Day 3\tPart 1: {}\t Part 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn day1() {
        assert_eq!(part1(INPUT), 357);
    }
    #[test]
    fn day2() {
        assert_eq!(part2(INPUT), 3121910778619);
    }
}
