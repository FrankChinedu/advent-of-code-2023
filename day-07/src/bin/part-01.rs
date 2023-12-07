use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = parse_input(true);
    let num = process(input);
    println!("num={num}");
}

const LABELS: [(char, i32); 13] = [
    ('A', 13),
    ('K', 12),
    ('Q', 11),
    ('J', 10),
    ('T', 9),
    ('9', 8),
    ('8', 7),
    ('7', 6),
    ('6', 5),
    ('5', 4),
    ('4', 3),
    ('3', 2),
    ('2', 1),
];

fn get_lables() -> HashMap<char, usize> {
    LABELS
        .iter()
        .fold(HashMap::new(), |mut hash, (label, stenght)| {
            hash.insert(*label, *stenght as usize);
            hash
        })
}

fn process(input: Vec<&str>) -> usize {
    let mut hands = input
        .iter()
        .map(|x| {
            let vec = x.split(' ').collect::<Vec<_>>();
            let hand_str = vec[0];
            let bid = vec[1].parse::<usize>().expect("get u8");
            let cards = hand_str.chars().map(Card::new).collect::<Vec<_>>();
            Hand::new(cards, bid, hand_str)
        })
        .collect::<Vec<_>>();
    hands.sort_by(|a, b| a.hand_type_strength.cmp(&b.hand_type_strength));

    let mut hands_group_hash: HashMap<usize, Vec<Hand>> = HashMap::new();

    for hand in hands {
        match hands_group_hash.get_mut(&hand.hand_type_strength) {
            Some(group) => {
                group.push(hand);
                group.sort_by(|a, b| {
                    let card_b = &b.cards[0];
                    let mut a_index = 0_usize;

                    for (i, val2) in a.cards.iter().enumerate() {
                        match val2.strength {
                            num if num == card_b.strength => continue,
                            _ => {
                                a_index = i;
                                break;
                            }
                        }
                    }
                    let card_a = &a.cards[a_index];
                    card_a.cmp(card_b)
                })
            }
            None => {
                hands_group_hash.insert(hand.hand_type_strength, vec![hand]);
            }
        }
    }

    let mut hand_group_arr: [Option<Vec<Hand>>; 8] = Default::default();

    for (index, hands) in hands_group_hash {
        hand_group_arr[index] = Some(hands);
    }

    hand_group_arr
        .iter()
        .filter_map(|x| x.as_ref())
        .flatten()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index + 1))
        .sum::<usize>()
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct Hand<'a> {
    cards: Vec<Card>,
    bid: usize,
    hand_str: &'a str,
    hand_type_strength: usize,
    hand_type: Type,
    card_strength: usize,
}

impl<'a> Hand<'a> {
    fn new(cards: Vec<Card>, bid: usize, hand_str: &'a str) -> Self {
        let mut hand = Self {
            cards,
            bid,
            hand_str,
            ..Default::default()
        };
        hand.set_hand_and_type(hand_str);
        hand.set_card_strenth();
        hand
    }

    fn set_hand_and_type(&mut self, hand_str: &str) {
        let (type_t, strength) = self.get_type_and_strength(hand_str);
        self.hand_type_strength = strength;
        self.hand_type = type_t;
    }

    fn set_card_strenth(&mut self) {
        let strength = &self.cards.iter().fold(0, |a, card| card.strength + a);
        self.card_strength = *strength;
    }

    fn get_type_and_strength(&self, input: &str) -> (Type, usize) {
        let mut hash: HashMap<char, usize> = HashMap::new();
        for chars in input.chars() {
            match hash.get(&chars) {
                Some(val) => hash.insert(chars, val + 1),
                None => hash.insert(chars, 1),
            };
        }

        let mut hash_arr = vec![];

        for val in hash.values() {
            hash_arr.push(*val);
        }

        hash_arr.sort_by(|a, b| b.cmp(a));
        let hash_arr = hash_arr;

        match hash_arr[..] {
            [_first] => (Type::FiveOfAKind, 7),
            [a, b] => {
                if a == 4 && b == 1 {
                    (Type::FourOfAKind, 6)
                } else {
                    (Type::FullHouse, 5)
                }
            }
            [a, b, c] => {
                if a == 3 && b == 1 && c == 1 {
                    (Type::ThreeOfAKind, 4)
                } else {
                    (Type::TwoPair, 3)
                }
            }
            [_a, _b, _c, _d] => (Type::OnePair, 2),
            _ => (Type::HighCard, 1),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Default, PartialEq, Eq)]
struct Card {
    label: char,
    strength: usize,
}

impl Card {
    fn new(label: char) -> Self {
        let strength = *get_lables().get(&label).expect("has strength");
        Self { label, strength }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength.cmp(&other.strength)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Default)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    #[default]
    HighCard,
}

fn parse_input<'a>(is_test_input: bool) -> Vec<&'a str> {
    let input = if is_test_input {
        include_str!("./test.txt")
    } else {
        include_str!("./input.txt")
    };
    input.lines().collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        let input = parse_input(true);
        assert_eq!(6440, process(input))
    }

    fn get_hand(hand_str: &str) -> Hand {
        let vec_str = hand_str.chars().map(Card::new).collect::<Vec<_>>();
        let bid = 456;
        Hand::new(vec_str, bid, hand_str)
    }

    #[test]
    fn is_five_of_a_kind() {
        let hand_str = "AAAAA";
        let hand = get_hand(hand_str);
        let hand_type = hand.hand_type;
        assert_matches::assert_matches!(hand_type, Type::FiveOfAKind);
    }

    #[test]
    fn is_third_of_a_kind() {
        let hand_str = "TTT98";
        let hand = get_hand(hand_str);
        let hand_type = hand.hand_type;
        assert_matches::assert_matches!(hand_type, Type::ThreeOfAKind);
    }

    #[test]
    fn is_high_card() {
        let hand_str = "23456";
        let hand = get_hand(hand_str);
        let hand_type = hand.hand_type;
        assert_matches::assert_matches!(hand_type, Type::HighCard);
    }
}
