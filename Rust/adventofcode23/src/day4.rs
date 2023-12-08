use std::collections::HashMap;

type Number = usize;

struct Card {
    id: usize,
    winning_numbers: Vec<Number>,
    numbers: Vec<Number>,
    points: Option<usize>,
}

impl Card {
    fn get_matching_numbers(&self) -> Vec<Number> {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .map(|n| *n)
            .collect::<Vec<Number>>()
    }

    fn get_points(&mut self) -> usize {
        if let Some(points) = self.points {
            return points;
        }
        let winning_numbers = self.get_matching_numbers();
        if winning_numbers.len() == 0 {
            return 0;
        }
        let result = 2_usize.pow(winning_numbers.len() as u32 - 1);
        self.points = Some(result);
        result
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let most_left_split = value.split(": ").collect::<Vec<&str>>();
        let id = most_left_split[0].split(" ").collect::<Vec<&str>>();
        let id = id.last().unwrap();

        let numbers = most_left_split[1].split(" | ").collect::<Vec<&str>>();
        let winning_numbers = numbers[0]
            .split(" ")
            .filter(|&n| !n.is_empty())
            .collect::<Vec<&str>>();
        let numbers = numbers[1]
            .split(" ")
            .filter(|&n| !n.is_empty())
            .collect::<Vec<&str>>();

        Card {
            id: id.parse::<usize>().unwrap(),
            winning_numbers: winning_numbers
                .iter()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
            numbers: numbers
                .iter()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
            points: None,
        }
    }
}

struct Cards(Vec<Card>);

struct CopiedCards {
    copies: HashMap<usize, usize>,
}

impl CopiedCards {
    fn get_amount_cards(&mut self) -> usize {
        self.copies.values().sum::<usize>()
    }
}

impl From<Vec<Card>> for Cards {
    fn from(value: Vec<Card>) -> Self {
        Cards(value)
    }
}

impl From<&str> for Cards {
    fn from(value: &str) -> Self {
        value
            .lines()
            .map(|l| Card::from(l))
            .collect::<Vec<Card>>()
            .into()
    }
}

impl Cards {
    fn get_points(&mut self) -> usize {
        self.0.iter_mut().map(|c| c.get_points()).sum()
    }

    fn calculate_copies(self) -> CopiedCards {
        let mut copies = HashMap::new();

        for id in 1..=self.0.len() {
            copies.insert(id, 1);
        }

        for card in &self.0 {
            let id = card.id;
            let numbers = card.get_matching_numbers().len();

            let current_card_count = *copies.get(&id).unwrap_or(&0);
            for copy_id in id + 1..=id + numbers {
                let count = copies.entry(copy_id).or_insert(1);
                *count += current_card_count;
            }
        }

        CopiedCards { copies }
    }
}

fn part1(input: &str) -> usize {
    Cards::from(input).get_points()
}

fn part2(input: &str) -> usize {
    Cards::from(input).calculate_copies().get_amount_cards()
}

pub fn day() {
    let input = include_str!("../input/day4.txt");
    println!(
        "Day 4\t Part 1: {}\t Part 2: {}",
        part1(input),
        part2(input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn test_test2() {
        assert_eq!(part2(INPUT), 30);
    }
}
