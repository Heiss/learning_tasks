use std::str::FromStr;

type Location = u32;

impl FromStr for UnsortedLocations {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arr: Vec<(Location, Location)> = s
            .lines()
            .map(|line| line.split("   "))
            .map(|mut arr| (arr.next().unwrap(), arr.next().unwrap()))
            .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
            .collect();
        let mut left = Vec::new();
        let mut right = Vec::new();

        for a in arr {
            left.push(a.0);
            right.push(a.1);
        }

        Ok(UnsortedLocations { left, right })
    }
}

impl UnsortedLocations {
    fn get_similarities(&self) -> SimilarityLocations {
        let mut similarities = Vec::new();

        for left in &self.left {
            let appears_times = self.right.iter().filter(|r| left == *r).count() as u32;
            similarities.push((*left, appears_times));
        }

        SimilarityLocations(similarities)
    }

    fn sort(mut self) -> SortedLocations {
        self.left.sort();
        self.right.sort();

        SortedLocations {
            left: self.left,
            right: self.right,
        }
    }
}

struct UnsortedLocations {
    left: Vec<Location>,
    right: Vec<Location>,
}

struct SortedLocations {
    left: Vec<Location>,
    right: Vec<Location>,
}

impl SortedLocations {
    fn get_distances(&self) -> Vec<u32> {
        self.left
            .iter()
            .zip(self.right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .collect()
    }
}

struct SimilarityLocations(Vec<(Location, u32)>);

impl SimilarityLocations {
    fn get_score(&self) -> u32 {
        self.0.iter().map(|(l, s)| l * s).sum::<u32>()
    }
}

fn part1(input: &str) -> usize {
    UnsortedLocations::from_str(input)
        .unwrap()
        .sort()
        .get_distances()
        .iter()
        .sum::<u32>() as usize
}

fn part2(input: &str) -> usize {
    UnsortedLocations::from_str(input)
        .unwrap()
        .get_similarities()
        .get_score() as usize
}

pub fn day() -> String {
    let input = include_str!("../input/day1.txt");
    format!("Day 1\tPart 1: {}\t Part 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        assert_eq!(part1(INPUT), 11);
    }
    #[test]
    fn day2() {
        const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        assert_eq!(part2(INPUT), 31);
    }
}
