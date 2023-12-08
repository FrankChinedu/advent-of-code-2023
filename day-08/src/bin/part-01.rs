use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let num = process(input);
    println!("num:{num}");
}

fn process(input: &str) -> usize {
    let input = input.lines().collect::<Vec<_>>();
    let clues = input.first().expect("msg").to_owned().to_owned();
    let nodes = input.get(2..).expect("msg");

    let mut direction_map = HashMap::new();
    let mut start_at = "AAA".to_owned();
    let find = "ZZZ".to_owned();

    for node in nodes {
        let dir = node.split('=').collect::<Vec<_>>();
        let name = dir[0].trim().to_owned();

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

    let mut steps = 0;

    'out: loop {
        for clue in clues.chars() {
            let dir = direction_map.get(&start_at).expect("msg");
            steps += 1;
            if clue == 'L' {
                start_at = dir.left.to_owned();
            } else {
                start_at = dir.right.to_owned();
            }

            if start_at == find {
                break 'out;
            }
        }
    }
    steps
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
