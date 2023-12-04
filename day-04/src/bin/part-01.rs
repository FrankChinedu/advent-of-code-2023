fn main() {
    let lines = include_str!("./input.txt");
    let num = process(lines);
    println!("num -- {:?}", num);
}

fn process(input: &str) -> i32 {
    input
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
        .filter(|x| !x.is_empty())
        .map(|x| {
            let cal = |n: i32| 2i32.pow((n - 1) as u32);
            cal(x.len() as i32)
        })
        .sum::<i32>()
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
        assert_eq!(13, process(input))
    }
}
