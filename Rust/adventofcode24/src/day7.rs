use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Operators {
    Add,
    Mul,
    Concat,
}

impl Operators {
    fn op(&self, left: &usize, right: &usize) -> usize {
        match self {
            Operators::Add => left + right,
            Operators::Mul => left * right,
            Operators::Concat => {
                let mut v = left.to_string();
                v.push_str(&right.to_string());
                v.parse::<usize>().unwrap()
            }
        }
    }

    fn ops(nums: &Vec<usize>, ops: &Vec<Operators>) -> usize {
        if ops.len() != nums.len() - 1 {
            panic!("Something is off");
        }

        let mut nums = nums.iter();
        let mut res = *nums.next().unwrap();

        for o in ops {
            res = o.op(&res, &nums.next().unwrap());
        }

        res
    }

    fn int_to_bit_array(num: usize, n: usize) -> Vec<u8> {
        let mut bits: Vec<u8> = Vec::with_capacity(n);

        for i in (0..n).rev() {
            // Extract the i-th bit and push it to the vector
            bits.push(((num >> i) & 1) as u8);
        }

        bits
    }

    fn get_ops_combinations(n: usize) -> Vec<Vec<Operators>> {
        let mut res = Vec::new();

        for i in 0..2_usize.pow(n as u32) {
            let v = Self::int_to_bit_array(i, n)
                .iter()
                .map(|b| {
                    if *b == 0 {
                        Operators::Add
                    } else if *b == 1 {
                        Operators::Mul
                    } else {
                        panic!("Should not be here!");
                    }
                })
                .collect();
            res.push(v);
        }

        res
    }

    fn get_ops_combinations_with_concat(n: usize) -> Vec<Vec<Operators>> {
        let mut res = Vec::new();
        for combi in Self::get_ops_combinations(n) {
            for i in 0..2_usize.pow(n as u32) {
                let a: Vec<Operators> = Self::int_to_bit_array(i, n)
                    .into_iter()
                    .zip(&combi)
                    .map(|bitops| match bitops {
                        (0, ops) => ops.clone(),
                        (1, _) => Operators::Concat,
                        _ => panic!("Should not happen!"),
                    })
                    .collect();
                res.push(a);
            }
        }

        res
    }
}

#[derive(Clone)]
struct Calibrations {
    search_value: usize,
    numbers: Vec<usize>,
}

impl FromStr for Calibrations {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut l = line.split(":");
        let val = l.next().unwrap().parse::<usize>().unwrap();
        let nums: Vec<usize> = l
            .flat_map(|v| v.split_whitespace().map(|v| v.parse::<usize>().unwrap()))
            .collect();

        if nums.len() == 0 {
            panic!("No numbers entered");
        }

        Ok(Calibrations {
            search_value: val,
            numbers: nums,
        })
    }
}

impl Calibrations {
    fn is_possibly_true(&self) -> bool {
        let operations: Vec<Vec<Operators>> =
            Operators::get_ops_combinations(self.numbers.len() - 1);

        for o in operations {
            if self.search_value == Operators::ops(&self.numbers, &o) {
                return true;
            }
        }

        false
    }

    fn is_really_true(&self) -> bool {
        let operations: Vec<Vec<Operators>> =
            Operators::get_ops_combinations_with_concat(self.numbers.len() - 1);

        for o in operations.iter() {
            if self.search_value == Operators::ops(&self.numbers, o) {
                return true;
            }
        }

        false
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|v| Calibrations::from_str(v).unwrap())
        .filter(Calibrations::is_possibly_true)
        .map(|v| v.search_value)
        .sum()
}

fn part2(input: &str) -> usize {
    // Better approach would be to calculate backwards: Check if the last number is a divisor
    // of the current searched value.
    // If it is, calculate the number and go one number backward.
    // If not, subtracts it and go one number backwards.
    // With this approach, you do not have to calculate ALL combinations.
    let callibrations = input.lines().map(|v| Calibrations::from_str(v).unwrap());

    let mut concat_vals = Vec::new();
    let mut sum = 0;
    for p in callibrations.clone() {
        if p.is_possibly_true() {
            sum += p.search_value
        } else {
            concat_vals.push(p.clone());
        }
    }

    sum + concat_vals
        .iter()
        .filter(|c| c.is_really_true())
        .map(|v| v.search_value)
        .sum::<usize>()
}

pub fn day() -> String {
    let input = include_str!("../input/day7.txt");
    format!("Day 7\tPart 1: {}\t Part 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
    #[test]
    fn day1() {
        assert_eq!(part1(INPUT), 3749);
    }
    #[test]
    fn day2() {
        assert_eq!(part2(INPUT), 11387);
    }
}
