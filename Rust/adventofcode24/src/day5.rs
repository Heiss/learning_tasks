use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

/// Place the rules in opposite meaning. Which number (value) should not be left of another number (key)?
#[derive(Debug)]
struct PageOrderingRules(HashMap<usize, Vec<usize>>);

impl FromStr for PageOrderingRules {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        for line in s.lines() {
            let mut s = line.split("|");
            let left = s.next().unwrap().parse::<usize>().unwrap();
            let right = s.next().unwrap().parse::<usize>().unwrap();
            map.entry(left).or_default().push(right);
        }
        Ok(PageOrderingRules(map))
    }
}

impl PageOrderingRules {
    fn check_order(&self, pages: &Pages) -> bool {
        let mut seen_number = Vec::new();
        let empty_vec = Vec::new();
        for n in &pages.0 {
            let not_allowed_page_numbers = if let Some(v) = self.0.get(&n) {
                v
            } else {
                &empty_vec
            };
            if seen_number
                .iter()
                .any(|v| not_allowed_page_numbers.contains(v))
            {
                return false;
            }
            seen_number.push(*n);
        }
        true
    }

    fn incorrect_orders<'a>(&self, pages: &'a Vec<Pages>) -> Vec<IncorrectOrderPages<'a>> {
        pages
            .iter()
            .filter(|&p| !self.check_order(p))
            .map(|p| IncorrectOrderPages(p))
            .collect()
    }

    fn correct_order<'a>(&self, pages: &IncorrectOrderPages<'a>) -> Pages {
        let mut new_pages = pages.0 .0.clone();
        new_pages.sort_by(|a, b| {
            if let Some(v) = self.0.get(&a) {
                if v.contains(b) {
                    return Ordering::Less;
                }
            }
            if let Some(v) = self.0.get(&b) {
                if v.contains(a) {
                    return Ordering::Greater;
                }
            }
            Ordering::Equal
        });
        Pages(new_pages)
    }
}

#[derive(Debug)]
struct Pages(Vec<usize>);

impl FromStr for Pages {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pages(
            s.lines()
                .flat_map(|v| v.split(","))
                .map(|v| v.parse::<usize>().unwrap())
                .collect(),
        ))
    }
}

impl Pages {
    fn get_middle_number(&self) -> usize {
        let n = self.0.len() / 2;
        self.0[n]
    }
}

struct IncorrectOrderPages<'a>(&'a Pages);

struct Manual {
    book: Vec<Pages>,
    rules: PageOrderingRules,
}

impl FromStr for Manual {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split("\n\n");
        let rules = PageOrderingRules::from_str(s.next().unwrap())?;
        let pages = s
            .next()
            .unwrap()
            .lines()
            .map(|v| Pages::from_str(v).unwrap())
            .collect();
        Ok(Self { rules, book: pages })
    }
}

fn part1(input: &str) -> usize {
    let man = Manual::from_str(input).unwrap();
    man.book
        .iter()
        .filter(|p| man.rules.check_order(p))
        .map(|p| p.get_middle_number())
        .sum()
}

fn part2(input: &str) -> usize {
    let man = Manual::from_str(input).unwrap();
    let incorrect_orders = man.rules.incorrect_orders(&man.book);
    incorrect_orders
        .iter()
        .map(|v| man.rules.correct_order(v))
        .map(|v| v.get_middle_number())
        .sum()
}

pub fn day() -> String {
    let input = include_str!("../input/day5.txt");
    format!("Day 5\tPart 1: {}\t Part 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
    #[test]
    fn day1() {
        assert_eq!(part1(INPUT), 143);
    }
    #[test]
    fn day2() {
        assert_eq!(part2(INPUT), 123);
    }
}
