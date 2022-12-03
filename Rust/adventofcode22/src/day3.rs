use itertools::Itertools;

const ALPHABET: [&str; 52] = alphabet::get_alphabet();

mod alphabet {
    pub const fn get_alphabet() -> [&'static str; 52] {
        [
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H",
            "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y",
            "Z",
        ]
    }
}

struct Backpack<'a> {
    items: Vec<Item<'a>>,
}

impl<'a> TryFrom<&'a str> for Backpack<'a> {
    type Error = &'a str;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let mut items = Vec::new();
        for line in input.split("") {
            if let Ok(item) = Item::try_from(line) {
                items.push(item);
            }
        }
        Ok(Backpack { items })
    }
}

impl<'a> Backpack<'a> {
    /// Returns the sum of priority of all items which are contained in the first half and the second half.
    fn get_priority_sum(&self) -> u64 {
        let len = &self.items.len();
        let fst_half = &self.items[..len / 2];
        let scnd_half = &self.items[len / 2..];

        fst_half
            .iter()
            .filter(|item| scnd_half.contains(item))
            .dedup()
            .fold(0, |acc, item| acc + item.get_priority())
    }

    /// Returns all items, which are contained in all three backpacks.
    fn get_common_items(&self, other: &Self, third: &Self) -> Vec<&Item<'a>> {
        let mut common_items = vec![];
        for item in &self.items {
            if other.items.contains(item)
                && third.items.contains(item)
                && !common_items.contains(&item)
            {
                common_items.push(item);
            }
        }
        common_items
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Item<'a>(&'a str);

impl<'a> Item<'a> {
    fn get_priority(&self) -> u64 {
        ALPHABET
            .iter()
            .enumerate()
            .find(|(_i, &letter)| letter == self.0)
            .unwrap()
            .0 as u64
            + 1
    }
}

impl<'a> TryFrom<&'a str> for Item<'a> {
    type Error = &'static str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if ALPHABET.contains(&value) {
            Ok(Self(value))
        } else {
            Err("Invalid item")
        }
    }
}

fn day3_part1(text: &str) -> u64 {
    let mut backpacks = vec![];
    for line in text.lines() {
        let backpack = Backpack::try_from(line).unwrap();
        backpacks.push(backpack);
    }

    backpacks
        .iter()
        .map(|backpack| backpack.get_priority_sum())
        .sum()
}

fn day3_part2(text: &str) -> u64 {
    let mut backpacks: Vec<Vec<Backpack>> = vec![];
    let mut iter = text.lines();
    while let (Some(v1), Some(v2), Some(v3)) = (iter.next(), iter.next(), iter.next()) {
        backpacks.push(vec![
            Backpack::try_from(v1).expect("Invalid input"),
            Backpack::try_from(v2).expect("Invalid input"),
            Backpack::try_from(v3).expect("Invalid input"),
        ]);
    }
    backpacks
        .iter()
        .map(|backpacks_line| {
            let fst = &backpacks_line[0];
            let scnd = &backpacks_line[1];
            let third = &backpacks_line[2];
            fst.get_common_items(scnd, third)
                .iter()
                .map(|item| item.get_priority())
                .sum::<u64>()
        })
        .sum()
}

pub fn day3(text: &str) {
    println!("Day 3 part 1: {}", day3_part1(text));
    println!("Day 3 part 2: {}", day3_part2(text));
}

#[cfg(test)]
mod tests {
    use super::{day3_part1, day3_part2};

    const TEST_INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn fixed_tests1() {
        assert_eq!(day3_part1(TEST_INPUT), 157);
    }

    #[test]
    fn fixed_tests2() {
        assert_eq!(day3_part2(TEST_INPUT), 70);
    }
}
