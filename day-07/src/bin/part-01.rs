use std::collections::HashMap;

fn main() {
    let input = parse_input(true);
    let num = process(input);
    println!("num={num}");
}

fn process(input: Vec<&str>) -> usize {
    let labels = [
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
    let labels = labels
        .iter()
        .fold(HashMap::new(), |mut hash, (label, stenght)| {
            hash.insert(*label, *stenght as usize);
            hash
        });

    let hands = input
        .iter()
        .map(|x| {
            let vec = x.split(' ').collect::<Vec<_>>();
            let hand_str = vec[0];
            let bid = vec[1].parse::<usize>().expect("get u8");
            let cards = hand_str
                .chars()
                .map(|label| {
                    let strength = *labels.get(&label).expect("has strength");
                    Card { label, strength }
                })
                .collect::<Vec<_>>();
            Hand::new(cards, bid, hand_str)
        })
        .collect::<Vec<_>>();

    dbg!(hands);

    // println!("hands={hands:?}");
    0
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct Hand<'a> {
    cards: Vec<Card>,
    bid: usize,
    hand_str: &'a str,
    hand_type_strength: usize,
    hand_type: Type,
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
        hand
    }

    fn set_hand_and_type(&mut self, hand_str: &str) {
        let (type_t, strength) = self.get_type_and_strength(hand_str);
        self.hand_type_strength = strength;
        self.hand_type = type_t;
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

        // for (key, val) in &hash {
        //     println!("key={key} val={val}")
        // }

        for val in hash.values() {
            hash_arr.push(*val);
        }

        hash_arr.sort_by(|a, b| b.cmp(a));
        let hash_arr = hash_arr;
        // println!(" ");
        // println!("sorted hash ={:?}", hash_arr);

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
#[derive(Debug, Default)]
struct Card {
    label: char,
    strength: usize,
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
}
