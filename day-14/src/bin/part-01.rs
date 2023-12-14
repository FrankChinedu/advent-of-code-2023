fn main() {
    let input = include_str!("./input.txt");
    println!("ans {}", process(input));
}

fn process(input: &str) -> usize {
    let lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut rock_locations = vec![];

    for (x, line) in lines.iter().enumerate() {
        if x != 0 {
            for (y, val) in line.iter().enumerate() {
                if val == &'O' {
                    let pos = (x, y);
                    rock_locations.push(pos);
                }
            }
        }
    }

    let mut lines = lines;
    for pos in &rock_locations {
        let y = pos.1;
        for x in (1..=pos.0).rev() {
            let up_index = x - 1;
            let tmp_char = lines[up_index][y];
            let current_char = lines[x][y];
            if tmp_char == '.' && current_char == 'O' {
                lines[up_index][y] = lines[x][y];
                lines[x][y] = tmp_char;
            }
        }
    }
    lines
        .iter_mut()
        .rev()
        .enumerate()
        .map(|(index, char)| {
            let num = index + 1;
            let count = char.iter().filter(|y| **y == 'O').count();
            num * count
        })
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("./test.txt");
        assert_eq!(136, process(input));
    }
}
