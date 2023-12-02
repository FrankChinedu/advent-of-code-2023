fn main() {
    let input = include_str!("./input.txt");

    let num = process(input);
    println!("num -- {:?}", num);
}

fn process(input: &str) -> u32 {
    let lines = input
        .lines()
        .map(|x| x.split(':').skip(1).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!(" ");
    println!("input {:?}", lines);

    let mut arry_multiples = vec![];
    for (_index, line) in lines.iter().enumerate() {
        let sep = line.join("");
        let sep = sep.split(';').collect::<Vec<_>>();

        let (mut red, mut green, mut blue) = (("red", 0), ("green", 0), ("blue", 0));

        for (it_index, it) in sep.iter().enumerate() {
            let ab = it.split(',').map(|x| x.trim()).collect::<Vec<_>>();

            for bc in ab {
                let split = bc.split(' ').collect::<Vec<_>>();
                let num = split
                    .first()
                    .expect("must be num")
                    .parse::<u32>()
                    .expect("is number");
                let color = split.last().expect("must be num");

                if color == &red.0 {
                    red.1 = if red.1 > num { red.1 } else { num }
                }
                if color == &green.0 {
                    green.1 = if green.1 > num { green.1 } else { num }
                }
                if color == &blue.0 {
                    blue.1 = if blue.1 > num { blue.1 } else { num }
                }
            }
            let num = red.1 * green.1 * blue.1;
            if it_index == sep.len() - 1 {
                arry_multiples.push(num);
            }
        }
    }
    arry_multiples.iter().sum()
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
        assert_eq!(2286, process(input))
    }
}
