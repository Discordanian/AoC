use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, PartialEq, Eq)]
pub struct Card {
    value: u32,
    disp: char,
}

#[derive(Clone, Debug, Ord, PartialOrd, PartialEq, Eq)]
pub struct HandType {
    value1: u32,
    value2: u32,
}

#[derive(Clone, Debug, Ord, PartialOrd, PartialEq, Eq)]
pub struct Hand {
    hand_value1: u32,
    hand_value2: u32,
    bid: u32,
    cards: Vec<Card>,
    hand_type: HandType,
}

impl From<Vec<Card>> for HandType {
    fn from(cards: Vec<Card>) -> HandType {
        let hm: HashMap<u32, u32> = cards.clone().iter().fold(HashMap::new(), |mut map, card| {
            *map.entry(card.value).or_insert(0) += 1;
            map
        });
        let max_val: u32 = *hm.iter().map(|x| x.1).max().unwrap();
        let num_jokers: u32 = cards.iter().filter(|card| card.disp == 'J').count() as u32;
        let v1 = match (hm.len() as u32, max_val) {
            (1, _) => 7, // 5 of a kind
            (2, 4) => 6, // 4 of a kind
            (2, _) => 5, // Full house
            (3, 3) => 4, // 3 of a kind
            (3, 2) => 3, // 2 pair
            (4, 2) => 2, // 1 pair
            (5, _) => 1, // high card
            (_, _) => panic!("What combo is this?"),
        };
        let v2 = match (v1, num_jokers) {
            (6, 1) => 7, // 4 of a kind becomes 5 of a kind
            (4, 1) => 6, // 3 of a kind becomes 4 of a kind
            (3, 1) => 5, // 2 pair becomes full house
            (2, 1) => 4, // pair becomes 3 of a kind
            (1, 1) => 2, // high card becomes pair
            (5, 2) => 7, // full house becomes 5 of a kind
            (3, 2) => 6, // two pair becomes 4 of a kind
            (2, 2) => 4, // 1 pair becomes 3 of a kind
            (5, 3) => 7, // full house becomes 5 of a kind
            (4, 3) => 6, // 3 of  kind becomes 4 of a kind
            (_, 4) => 7, // 5 of a kind for 4 jokers
            (_, _) => v1,
        };

        HandType {
            value1: v1,
            value2: v2,
        }
    }
}

impl From<&str> for Hand {
    fn from(line: &str) -> Hand {
        let parts: Vec<&str> = line.split(" ").collect();
        let c: Vec<Card> = parts[0].chars().map(Card::from).collect();
        let b: u32 = parts[1].trim().parse().unwrap();
        let ht = HandType::from(c.clone());
        let hv1: u32 = c.clone().iter().fold(ht.value1, |mut acc, c| {
            acc = acc * 14 + c.value;
            acc
        });
        let hv2: u32 = c.clone().iter().fold(ht.value2, |mut acc, c| {
            acc = acc * 14
                + match c.value {
                    11 => 1,
                    _ => c.value,
                };
            acc
        });
        Hand {
            hand_value1: hv1,
            hand_value2: hv2,
            bid: b,
            cards: c.clone(),
            hand_type: ht,
        }
    }
}

impl From<char> for Card {
    fn from(ch: char) -> Card {
        let v = match ch {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid character for Card"),
        };
        Card { value: v, disp: ch }
    }
}

pub fn process_part1(input: &str) -> u32 {
    let mut hands: Vec<Hand> = input.lines().map(Hand::from).collect();
    hands.sort_by(|a, b| a.hand_value1.cmp(&b.hand_value1));
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u32 * h.bid)
        .sum()
}

pub fn process_part2(input: &str) -> u32 {
    let mut hands: Vec<Hand> = input.lines().map(Hand::from).collect();
    hands.sort_by(|a, b| a.hand_value2.cmp(&b.hand_value2));
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u32 * h.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 6440);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 5905);
    }
}
