fn main() {
    let input = include_str!("./input.txt");
    let num = process(input);
    println!("num -- {:?}", num);
}

#[derive(Debug, Clone)]
struct Card {
    name: String,
    matching_numbers: usize,
    copies: usize,
    original_count: usize,
    total: usize,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            matching_numbers: 0,
            copies: 0,
            original_count: 1,
            total: 1,
            name: "Card".into(),
        }
    }
}

impl Card {
    fn update_matching_numbers(&mut self, value: usize) {
        self.matching_numbers = value
    }

    fn update_copies(&mut self, value: usize) {
        self.copies += value;
        self.calculate_total();
    }

    fn calculate_total(&mut self) {
        self.total = self.copies + self.original_count
    }

    fn update_name(&mut self, value: usize) {
        let name = format!("{} {value}", self.name);
        self.name = name;
    }
}

fn process(input: &str) -> i32 {
    let mut cards = input
        .lines()
        .map(|x| x.split(':').collect::<Vec<_>>())
        .map(|x| *x.last().expect(""))
        .map(|x| x.split('|').collect::<Vec<_>>())
        .map(|x| {
            let first = x
                .first()
                .expect("some")
                .split(' ')
                .filter(|x| x != &"")
                .collect::<Vec<_>>();
            // println!("first {first:?}");
            let last = x
                .last()
                .expect("some")
                .split(' ')
                .filter(|x| x != &"")
                .collect::<Vec<_>>();
            let ve = first
                .iter()
                .filter(|x| last.contains(x))
                .cloned()
                .collect::<Vec<_>>();
            ve
        })
        // .filter(|x| !x.is_empty())
        .map(|x| x.len())
        .enumerate()
        .map(|(index, x)| {
            let mut card = Card::default();
            card.update_matching_numbers(x);
            card.update_name(index + 1);
            card
        })
        .collect::<Vec<_>>();

    for index in 0..cards.len() {
        let current_card = &cards[index];
        let index_plus_one = index + 1;

        let total = current_card.total;

        for i in 0..current_card.matching_numbers {
            if let Some(card_instances) = cards.get_mut(index_plus_one..) {
                for (y, card_in) in card_instances.iter_mut().enumerate() {
                    if y == i {
                        card_in.update_copies(total);
                    }
                }
            }
        }
    }

    cards.iter().map(|x| x.total).sum::<usize>() as i32
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        assert_eq!(30, process(input))
    }
}
