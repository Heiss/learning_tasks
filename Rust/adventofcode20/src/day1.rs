struct ExpenseReport {
    entries: Vec<i32>,
}

impl TryFrom<&str> for ExpenseReport {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let entries = value
            .lines()
            .map(|line| line.parse::<i32>().expect("Invalid number"))
            .collect::<Vec<i32>>();
        Ok(ExpenseReport { entries })
    }
}

impl ExpenseReport {
    fn find_two_entries_that_sum_to(&self, target: i32) -> Option<(i32, i32)> {
        for (i, entry) in self.entries.iter().enumerate() {
            for other_entry in self.entries.iter().skip(i + 1) {
                if entry + other_entry == target {
                    return Some((*entry, *other_entry));
                }
            }
        }
        None
    }

    fn find_three_entries_that_sum_to(&self, target: i32) -> Option<(i32, i32, i32)> {
        for (i, entry) in self.entries.iter().enumerate() {
            for (j, other_entry) in self.entries.iter().enumerate().skip(i + 1) {
                for third_entry in self.entries.iter().skip(j + 1) {
                    if entry + other_entry + third_entry == target {
                        return Some((*entry, *other_entry, *third_entry));
                    }
                }
            }
        }
        None
    }
}

fn day1_part1(text: &str) -> u64 {
    let report = ExpenseReport::try_from(text).expect("Invalid input");
    let (a, b) = report
        .find_two_entries_that_sum_to(2020)
        .expect("No entries sums to 2020");
    (a * b) as u64
}

fn day1_part2(text: &str) -> u64 {
    let report = ExpenseReport::try_from(text).expect("Invalid input");
    let (a, b, c) = report
        .find_three_entries_that_sum_to(2020)
        .expect("No entries sums to 2020");
    (a * b * c) as u64
}

pub fn day1(text: &str) {
    println!("Day1 part 1: {}", day1_part1(text));
    println!("Day1 part 2: {}", day1_part2(text));
}

#[cfg(test)]
mod tests {
    use super::{day1_part1, day1_part2};

    #[test]
    fn fixed_tests() {
        let test_input = r#"1721
979
366
299
675
1456"#;

        assert_eq!(day1_part1(test_input), 514579);
        assert_eq!(day1_part2(test_input), 241861950);
    }
}
