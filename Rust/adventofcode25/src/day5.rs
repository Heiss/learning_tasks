use itertools::Itertools;
use std::str::FromStr;

struct IngredientId(usize);
struct IngredientIds(Vec<IngredientId>);

impl FromStr for IngredientId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse().map(IngredientId).unwrap())
    }
}

impl FromStr for IngredientIds {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(IngredientIds(
            s.lines()
                .map(IngredientId::from_str)
                .map(Result::unwrap)
                .collect(),
        ))
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct IngredientRange {
    min: usize,
    max: usize,
}

struct IngredientRanges(Vec<IngredientRange>);

impl FromStr for IngredientRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split('-');
        Ok(IngredientRange {
            min: s.next().unwrap().parse().unwrap(),
            max: s.next().unwrap().parse().unwrap(),
        })
    }
}

impl IngredientRange {
    fn contains(&self, v: &IngredientId) -> bool {
        self.min <= v.0 && v.0 <= self.max
    }

    fn get_fresh_ingredients(&self) -> Vec<usize> {
        (self.min..=self.max).collect()
    }

    fn count_fresh_ingredients(&self) -> usize {
        self.max - self.min + 1
    }

    fn distinct(self, o: IngredientRange) -> IngredientRanges {
        let min = self.min.min(o.min);
        let max = self.max.max(o.max);

        // Check overlap:
        // self.max >= o.min && o.max >= self.min
        // We also merge touching ranges (e.g. 1-5 and 6-10) by using saturating_add(1)
        if self.max.saturating_add(1) >= o.min && o.max.saturating_add(1) >= self.min {
            IngredientRanges(vec![IngredientRange { min, max }])
        } else {
            IngredientRanges(vec![self, o])
        }
    }
}

impl FromStr for IngredientRanges {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(IngredientRanges(
            s.lines()
                .map(|v| IngredientRange::from_str(v).unwrap())
                .collect(),
        ))
    }
}
impl IngredientRanges {
    fn contains(&self, v: &IngredientId) -> bool {
        self.0.iter().any(|r| r.contains(v))
    }

    fn count_fresh_ingredients(&self, v: &IngredientIds) -> usize {
        v.0.iter().filter(|&v| self.contains(&v)).count()
    }

    fn count_all_fresh_ingredients(&mut self) -> usize {
        // Sort by start position to allow linear merge
        self.0.sort_by_key(|r| r.min);

        let mut merged: Vec<IngredientRange> = Vec::new();

        for range in &self.0 {
            if let Some(last) = merged.last_mut() {
                // If current range overlaps or touches the last one, merge them
                if range.min <= last.max.saturating_add(1) {
                    last.max = last.max.max(range.max);
                } else {
                    merged.push(*range);
                }
            } else {
                merged.push(*range);
            }
        }

        self.0 = merged;
        self.0.iter().map(|r| r.count_fresh_ingredients()).sum()
    }
}

struct Hall {
    ranges: IngredientRanges,
    ingredients: IngredientIds,
}

impl FromStr for Hall {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split("\n\n");
        Ok(Hall {
            ranges: IngredientRanges::from_str(s.next().unwrap())?,
            ingredients: IngredientIds::from_str(s.next().unwrap())?,
        })
    }
}

impl Hall {
    fn count_fresh_ingredients(&self) -> usize {
        self.ranges.count_fresh_ingredients(&self.ingredients)
    }

    fn count_all_fresh_ingredients(&mut self) -> usize {
        self.ranges.count_all_fresh_ingredients()
    }
}

fn part1(input: &str) -> usize {
    Hall::from_str(input).unwrap().count_fresh_ingredients()
}

fn part2(input: &str) -> usize {
    Hall::from_str(input).unwrap().count_all_fresh_ingredients()
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode25/day5.txt");
    format!("Day 5\tPart 1: {}\t Part 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn day1() {
        assert_eq!(part1(INPUT), 3);
    }
    #[test]
    fn day2() {
        assert_eq!(part2(INPUT), 14);
    }
}
