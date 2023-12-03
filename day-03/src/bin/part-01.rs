fn main() {
    let input = include_str!("./input.txt");
    let num = process(input);
    println!("num={num}")
}

fn process(input: &str) -> i32 {
    let directions = [
        (-1, 0, "up"),
        (1, 0, "down"),
        (-1, -1, "up_left"),
        (-1, 1, "up_right"),
        (1, -1, "down_left"),
        (1, 1, "down_right"),
        (0, -1, "left"),
        (0, 1, "right"),
    ];

    let lines = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| {
                    if x.is_ascii_punctuation() && x != '.' || x.is_numeric() {
                        x
                    } else {
                        ' '
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut num_group_vec = vec![];
    for (i, line) in lines.iter().enumerate() {
        let mut num_group = "".to_string();
        let mut has_seen_symbol = false;

        for (j, inner_line) in line.iter().enumerate() {
            if inner_line.is_ascii_punctuation() {
                continue;
            }

            for (_dir_index, dir) in directions.iter().enumerate() {
                let outer = i as isize + dir.0;
                let inner = j as isize + dir.1;
                let inner_line_length = line.len() as isize;
                let outer_line_length = lines.len() as isize;

                if outer.is_negative()
                    || inner.is_negative()
                    || inner > (inner_line_length - 1)
                    || outer > (outer_line_length - 1)
                {
                    continue;
                }

                let neighbor = lines[outer as usize][inner as usize];
                if inner_line == &' ' || inner_line.is_ascii_punctuation() {
                    has_seen_symbol = false;
                }
                if dir.2 == "left" && j == (inner_line_length - 1) as usize {
                    num_group = format!("{num_group}{inner_line}");

                    if has_seen_symbol && !num_group.trim().is_empty() {
                        num_group_vec.push(num_group.trim().to_string());
                    }
                    num_group = "".to_string();
                    has_seen_symbol = false;
                }

                if dir.2 == "right" {
                    num_group = format!("{num_group}{inner_line}");

                    if neighbor == ' ' {
                        if has_seen_symbol && !num_group.trim().is_empty() {
                            num_group_vec.push(num_group.trim().to_string());
                        }
                        num_group = "".to_string();
                        has_seen_symbol = false;
                    } else if neighbor.is_ascii_punctuation() && !num_group.trim().is_empty() {
                        has_seen_symbol = false;
                        num_group_vec.push(num_group.trim().to_string());
                        num_group = "".to_string();
                    }
                }
                if neighbor.is_ascii_punctuation() {
                    has_seen_symbol = true;
                }
            }
        }
    }
    num_group_vec
        .iter()
        .map(|x| x.parse::<i32>().expect("couldn't pass number"))
        .sum::<i32>()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        let input = r#"467..114..
...*......
..35...633
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(4361, process(input))
    }

    #[test]
    fn test_2() {
        let input = r#"*467..114.
...*......
..35=.633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(4361, process(input))
    }
}
