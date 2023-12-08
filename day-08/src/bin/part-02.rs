use indicatif::ProgressIterator;
use std::collections::HashMap;

#[allow(dead_code)]
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    let c = a * b;
    c / gcd(a, b)
}

fn main() {
    let input = include_str!("./input.txt");
    // let num = lcm(2, 3);
    let num = process(input);
    println!("num:{num}");
}

fn process(input: &str) -> usize {
    let input = input.lines().collect::<Vec<_>>();
    let clues = input.first().expect("msg").to_owned().to_owned();
    let nodes = input.get(2..).expect("msg");

    let mut direction_map = HashMap::new();
    let mut search_val = vec![];

    for node in nodes {
        let dir = node.split('=').collect::<Vec<_>>();
        let name = dir[0].trim().to_owned();
        if name.ends_with('A') {
            search_val.push(name.to_owned());
        }

        let second = dir[1].replace('(', " ").replace(')', "");
        let second = second
            .split(',')
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let direction = Direction::new(
            name.to_owned(),
            second[0].trim().to_owned(),
            second[1].trim().to_owned(),
        );
        direction_map.insert(name, direction);
    }

    let mut steps = 0_usize;
    println!("search_val ={:?}", search_val);

    let mut all_counts = vec![];

    for search_item in search_val.iter().progress() {
        let mut temp_item = search_item.to_owned();
        'inloop: loop {
            for clue in clues.chars() {
                let dir = direction_map.get(&temp_item).expect("msg");
                steps += 1;
                if clue == 'L' {
                    temp_item = dir.left.to_owned();
                } else {
                    temp_item = dir.right.to_owned();
                }

                if temp_item.ends_with('Z') {
                    all_counts.push(steps);
                    steps = 0;
                    break 'inloop;
                }
            }
        }
    }
    println!("all_counts:{:?}", all_counts);
    all_counts.iter().fold(1, |a, b| lcm(a, *b))
}

#[allow(dead_code)]
#[derive(Debug)]
struct Direction {
    name: String,
    left: String,
    right: String,
}

impl Direction {
    fn new(name: String, left: String, right: String) -> Self {
        Self { name, left, right }
    }
}

#[cfg(test)]
mod test {
    // use super::*;
    #[test]
    fn test_1() {}
}
