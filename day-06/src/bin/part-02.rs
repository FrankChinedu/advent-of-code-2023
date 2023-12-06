fn get_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .flat_map(|x| {
            x.split(':')
                .map(|x| x.replace(' ', ""))
                .filter(|x| x.parse::<usize>().is_ok())
                .collect::<Vec<_>>()
        })
        .map(|x| x.parse::<usize>().expect("should have"))
        .collect::<Vec<_>>()
}
fn main() {
    let input = include_str!("./input.txt");
    let input = get_input(input);
    println!("input={:?}", input);
    let time = input[0];
    let distance = input[1];

    let num = process(time, distance);

    println!("num={num}")
}

fn process(time: usize, distance: usize) -> usize {
    let hold = 1;
    let mut ways_to_win = vec![];
    let mut count = 0;
    for hold_val in hold..time {
        let distance_traveled = hold_val * (time - hold_val);
        if distance_traveled > distance {
            count += 1;
        }
    }
    ways_to_win.push(count);

    ways_to_win.iter().product()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        let input = r#"
        Time:      7  15   30
Distance:  9  40  200"#;
        let input = get_input(input);
        assert_eq!(71503, process(input[0], input[1]))
    }
}
