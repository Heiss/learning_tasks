use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[derive(Debug)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Joker,
}

impl Card {
    fn get_value(&self) -> usize {
        match self {
            Card::Ace => 14,
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Jack => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::Joker => 1,
        }
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::Ace),
            "2" => Ok(Card::Two),
            "3" => Ok(Card::Three),
            "4" => Ok(Card::Four),
            "5" => Ok(Card::Five),
            "6" => Ok(Card::Six),
            "7" => Ok(Card::Seven),
            "8" => Ok(Card::Eight),
            "9" => Ok(Card::Nine),
            "T" => Ok(Card::Ten),
            "J" => Ok(Card::Jack),
            "Q" => Ok(Card::Queen),
            "K" => Ok(Card::King),
            _ => Err(()),
        }
    }
}

type Bid = usize;

#[derive(Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn get_value(&self) -> usize {
        match self {
            HandType::HighCard => 1,
            HandType::OnePair => 2,
            HandType::TwoPair => 3,
            HandType::ThreeOfAKind => 4,
            HandType::FullHouse => 5,
            HandType::FourOfAKind => 6,
            HandType::FiveOfAKind => 7,
        }
    }

    fn next(&self) -> Self {
        match self {
            HandType::HighCard => HandType::OnePair,
            HandType::OnePair => HandType::ThreeOfAKind,
            HandType::TwoPair => HandType::FullHouse,
            HandType::ThreeOfAKind => HandType::FourOfAKind,
            HandType::FullHouse => HandType::FourOfAKind,
            HandType::FourOfAKind => HandType::FiveOfAKind,
            HandType::FiveOfAKind => panic!("No next hand type"),
        }
    }

    fn from_cards(cards: &Vec<Card>) -> Self {
        let mut counts = [0; 15];
        for card in cards {
            counts[card.get_value() - 1] += 1;
        }
        counts[Card::Joker.get_value() - 1] = 0;
        let mut pairs = 0;
        let mut triples = 0;
        let mut quadruples = 0;
        let mut quintuples = 0;
        for count in counts.iter() {
            match count {
                0 | 1 => (),
                2 => pairs += 1,
                3 => triples += 1,
                4 => quadruples += 1,
                5 => quintuples += 1,
                _ => panic!("Invalid hand {}", count),
            }
        }
        match (pairs, triples, quadruples, quintuples) {
            (0, 0, 0, 0) => HandType::HighCard,
            (1, 0, 0, 0) => HandType::OnePair,
            (2, 0, 0, 0) => HandType::TwoPair,
            (0, 1, 0, 0) => HandType::ThreeOfAKind,
            (1, 1, 0, 0) => HandType::FullHouse,
            (0, 0, 1, 0) => HandType::FourOfAKind,
            (0, 0, 0, 1) => HandType::FiveOfAKind,
            _ => panic!("Invalid hand"),
        }
    }

    fn from_cards_with_jokers(cards: &Vec<Card>) -> Self {
        let mut normal_type = HandType::from_cards(cards);

        let mut card_counts = [0; 15];
        for card in cards {
            card_counts[card.get_value() - 1] += 1;
        }
        let count_joker = std::mem::take(&mut card_counts[Card::Joker.get_value() - 1]);
        card_counts[Card::Joker.get_value() - 1] = 0;

        let max_appliable = 5 - card_counts.iter().max().unwrap();
        if count_joker == 5 {
            return HandType::FiveOfAKind;
        }

        for _ in 0..max_appliable.min(count_joker) {
            normal_type = normal_type.next();
        }

        normal_type
    }
}

impl Eq for HandType {}

impl PartialEq<Self> for HandType {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

impl PartialOrd<Self> for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_value().partial_cmp(&other.get_value())
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_value().cmp(&other.get_value())
    }
}

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

impl Eq for Card {}

impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_value().partial_cmp(&other.get_value())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_value().cmp(&other.get_value())
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: Bid,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect::<Vec<&str>>();

        let cards = split[0]
            .chars()
            .map(|c| c.to_string().parse::<Card>().unwrap())
            .collect();

        let bid = split[1].parse::<Bid>().unwrap();
        let hand_type = HandType::from_cards(&cards);

        Ok(Hand {
            cards,
            hand_type,
            bid,
        })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct HandJoker(Hand);

impl From<Hand> for HandJoker {
    fn from(hand: Hand) -> Self {
        let cards = hand
            .cards
            .into_iter()
            .map(|c| if c == Card::Jack { Card::Joker } else { c })
            .collect::<Vec<Card>>();

        let hand_type = HandType::from_cards_with_jokers(&cards);

        HandJoker(Hand {
            cards,
            hand_type,
            bid: hand.bid,
        })
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type == other.hand_type {
            self.cards.partial_cmp(&other.cards)
        } else {
            self.hand_type.partial_cmp(&other.hand_type)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            self.cards.cmp(&other.cards)
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

struct Game {
    hands: Vec<Hand>,
}

impl Game {
    fn get_total_winnings(&mut self) -> usize {
        self.hands.sort_unstable();
        self.hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) * hand.bid)
            .sum()
    }
}

struct GameJoker(Game);

impl From<Game> for GameJoker {
    fn from(game: Game) -> Self {
        let hands: Vec<HandJoker> = game.hands.into_iter().map(|h| h.into()).collect();
        let hands = hands.into_iter().map(|h| h.0).collect();
        GameJoker(Game { hands })
    }
}

impl DerefMut for GameJoker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for GameJoker {
    type Target = Game;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands = s
            .lines()
            .map(|line| line.parse::<Hand>().unwrap())
            .collect();
        Ok(Game { hands })
    }
}

fn part1(input: &str) -> usize {
    let mut game = input.parse::<Game>().unwrap();
    game.get_total_winnings()
}

fn part2(input: &str) -> usize {
    let game = input.parse::<Game>().unwrap();
    let mut game_joker = GameJoker::from(game);
    game_joker.get_total_winnings()
}

pub fn day() {
    let input = include_str!("../input/day7.txt");
    print!("Day 7\t");
    print!("Part 1: {}\t", part1(input));
    print!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 5905);
    }

    #[test]
    fn test_joker_strength() {
        let hand1 = HandJoker::from("QQQQ2 1".parse::<Hand>().unwrap());
        let hand2 = HandJoker::from("JKKK2 2".parse::<Hand>().unwrap());
        assert!(hand1 > hand2);
    }

    #[test]
    fn test_joker_prevelance_change() {
        let hand1 = "JJJJJ 1".parse::<Hand>().unwrap();
        let hand2 = "TTTTT 2".parse::<Hand>().unwrap();

        assert!(hand1 > hand2);
        assert!(HandJoker::from(hand1) < HandJoker::from(hand2));
    }
}
