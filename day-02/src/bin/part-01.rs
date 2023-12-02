fn main() {
    let lines = include_str!("./input.txt");
    let num = process(lines);
    println!("num -- {:?}", num);
}

fn process(input: &str) -> u32 {
    let (red, green, blue) = (("red", 12), ("green", 13), ("blue", 14));
    let lines = input
        .lines()
        .map(|x| x.split(':').skip(1).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ids = vec![];

    for (index, line) in lines.iter().enumerate() {
        let sep = line.join("");
        let sep = sep.split(';').collect::<Vec<_>>();

        let mut seen = true;

        'sep: for it in sep {
            let ab = it.split(',').map(|x| x.trim()).collect::<Vec<_>>();

            for bc in ab {
                let split = bc.split(' ').collect::<Vec<_>>();
                let num = split
                    .first()
                    .expect("must be num")
                    .parse::<i32>()
                    .expect("is number");
                let color = split.last().expect("must be num");

                if color == &red.0 && num > red.1 {
                    seen = false;
                    break 'sep;
                }
                if color == &green.0 && num > green.1 {
                    seen = false;
                    break 'sep;
                }
                if color == &blue.0 && num > blue.1 {
                    seen = false;
                    break 'sep;
                }
            }
        }

        if seen {
            ids.push((index + 1) as u32);
        };
    }

    ids.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(8, process(input))
    }
}
