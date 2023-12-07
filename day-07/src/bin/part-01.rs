use std::{cmp::Ordering, collections::HashMap};

fn main() {
    // let input = parse_input(false);
    let input = parse_input(false);
    let num = process(input);
    println!("num={num}");
}

const LABELS: [(char, i32); 13] = [
    ('A', 14),
    ('K', 13),
    ('Q', 12),
    ('J', 11),
    ('T', 10),
    ('9', 9),
    ('8', 8),
    ('7', 7),
    ('6', 6),
    ('5', 5),
    ('4', 4),
    ('3', 3),
    ('2', 2),
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

    hands.sort_by_key(|a| (a.hand_type, a.card_strength_tuple));

    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index + 1))
        .sum::<usize>()
}

#[allow(dead_code)]
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Hand<'a> {
    cards: Vec<Card>,
    bid: usize,
    hand_str: &'a str,
    hand_type_strength: usize,
    hand_type: Type,
    card_strength: usize,
    card_strength_tuple: (usize, usize, usize, usize, usize),
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
        hand.set_card_strength_tuple();
        hand
    }

    fn set_hand_and_type(&mut self, hand_str: &str) {
        let (type_t, strength) = self.get_type_and_strength(hand_str);
        self.hand_type_strength = strength;
        self.hand_type = type_t;
    }

    fn set_card_strength_tuple(&mut self) {
        let cards = &self.cards;
        self.card_strength_tuple = (
            cards[0].strength,
            cards[1].strength,
            cards[2].strength,
            cards[3].strength,
            cards[4].strength,
        )
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
            [_first] => (Type::FiveOfAKind, 6),
            [a, b] => {
                if a == 4 && b == 1 {
                    (Type::FourOfAKind, 5)
                } else {
                    (Type::FullHouse, 4)
                }
            }
            [a, b, c] => {
                if a == 3 && b == 1 && c == 1 {
                    (Type::ThreeOfAKind, 3)
                } else {
                    (Type::TwoPair, 2)
                }
            }
            [_a, _b, _c, _d] => (Type::OnePair, 1),
            _ => (Type::HighCard, 0),
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

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Type {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    #[default]
    HighCard = 0,
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
    fn is_five_of_a_kind_7() {
        let hand_str = "JJJJJ";
        let hand = get_hand(hand_str);
        let hand_type = hand.hand_type;
        assert_matches::assert_matches!(hand_type, Type::FiveOfAKind);
    }

    #[test]
    fn is_four_of_a_kind_6() {
        let hand_str = "66686";
        let hand = get_hand(hand_str);
        let hand_type = hand.hand_type;
        assert_matches::assert_matches!(hand_type, Type::FourOfAKind);
    }

    #[test]
    fn is_full_house_5() {
        let hand_str = "5555J";
        let hand = get_hand(hand_str);
        let hand_type = hand.hand_type;
        assert_matches::assert_matches!(hand_type, Type::FourOfAKind);
    }

    #[test]
    fn is_three_of_a_kind_4() {
        let hand_str = "T99J9";
        let hand = get_hand(hand_str);
        let hand_type = hand.hand_type;
        assert_matches::assert_matches!(hand_type, Type::ThreeOfAKind);
    }

    #[test]
    fn is_two_pair_3() {
        let hand_str = "JQKKQ";
        let hand = get_hand(hand_str);
        let hand_type = hand.hand_type;
        assert_matches::assert_matches!(hand_type, Type::TwoPair);
    }

    #[test]
    fn is_one_pair_2() {
        let hand_str = "443TK";
        let hand = get_hand(hand_str);
        let hand_type = hand.hand_type;
        assert_matches::assert_matches!(hand_type, Type::OnePair);
    }

    #[test]
    fn is_high_card_1() {
        let hand_str = "7KQ53";
        let hand = get_hand(hand_str);
        let hand_type = hand.hand_type;
        assert_matches::assert_matches!(hand_type, Type::HighCard);
    }
}
