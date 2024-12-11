use std::collections::HashMap;
use std::mem;
use std::str::FromStr;

struct Operation {
    arrangement: HashMap<usize, usize>,
}
impl Operation {
    fn op1(&mut self, num: usize) -> Vec<usize> {
        let res = if num == 0 {
            vec![1]
        } else if &num.to_string().len() % 2 == 0 {
            let len = (num.checked_ilog10().unwrap_or_default() + 1) / 2;
            let num_left = num / 10_usize.pow(len as u32);
            let num_right = num % 10_usize.pow(len as u32);
            vec![num_left, num_right]
        } else {
            vec![num * 2024]
        };
        res
    }

    fn blink(&mut self) {
        let mut tmp = HashMap::new();
        mem::swap(&mut tmp, &mut self.arrangement);
        for (num, count) in tmp.iter() {
            let arr = self.op1(*num);
            for a in arr {
                *self.arrangement.entry(a).or_default() += count;
            }
        }
    }

    fn count_stones(&self) -> usize {
        self.arrangement.values().sum()
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            arrangement: s.lines().next().unwrap().split_whitespace().fold(
                HashMap::new(),
                |mut res, v| {
                    res.insert(v.parse::<usize>().unwrap(), 1);
                    res
                },
            ),
        })
    }
}

fn part1(input: &str) -> usize {
    let mut arrangement = Operation::from_str(input).unwrap();
    for _ in 0..25 {
        arrangement.blink();
    }
    arrangement.count_stones()
}

fn part2(input: &str) -> usize {
    let mut arrangement = Operation::from_str(input).unwrap();
    for _ in 0..75 {
        arrangement.blink();
    }
    arrangement.count_stones()
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode24/day11.txt");
    format!(
        "Day 11\tPart 1: {}\t Part 2: {}",
        part1(input),
        part2(input)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"125 17"#;
    #[test]
    fn day1() {
        assert_eq!(part1(INPUT), 55312);
    }
    #[test]
    fn day2() {}
}
