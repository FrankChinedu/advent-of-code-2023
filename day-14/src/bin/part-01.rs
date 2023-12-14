fn main() {
    let input = include_str!("./test1.txt");
    // process(input);
    println!("ans {}", process(input));
}

// fn generate_pairs2(n: usize, k: usize) -> Vec<(usize, usize)> {
//     let mut pairs = Vec::new();

//     for i in 1..=n {
//         for j in (i + 1)..=std::cmp::min(i + k - 1, n) {
//             pairs.push((i, j));
//         }
//     }

//     pairs
// }
fn call(x: usize, y: usize, lines: &mut Vec<Vec<char>>) {
    if x != 0 {
        let up_index = x - 1;
        let up_char = &lines[up_index][y];
        let char = &lines[x][y];
        // print!("y {y} char{char}");
        if char == &'#' || up_char == &'#' || char == &'.' {
            return;
            // let num = x - 1;
            // return _call(num, y, lines);
        }
        {
            let tmp_char = lines[up_index][y];
            lines[up_index][y] = lines[x][y];
            lines[x][y] = tmp_char;

            // let num = x - 1;
            // call(num, y, lines)
        }
    }
    // println!(" ");
}

fn process(input: &str) -> usize {
    let lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let len = lines.len();
    let line_len = lines[0].len();

    println!(" line_len {line_len} len {len}");
    println!(" ");

    let mut lines = lines;
    for x in (1..len).rev() {
        println!(" ");
        println!(" x ={x}");
        for y in 0..line_len {
            print!("{y} ");
            let up_index = x as isize - 1;
            print!("y {y}  ===>> ");
            if x != 0 || up_index.is_positive() {
                for j in (0..=x).rev() {
                    call(j, y, &mut lines);
                    // print!("{j} ");
                }
                // call(x, y, &mut lines)
            }
            // println!(" ");
            println!(" ")
        }
    }
    // println!("  ");
    let lines: Vec<String> = lines.iter().map(|x| x.iter().collect()).collect();

    // println!(" ");

    for x in &lines {
        println!("{x:?}");
        // println!(" ");
    }

    println!("  ");
    // lines
    //     .iter_mut()
    //     .rev()
    //     .enumerate()
    //     .map(|(index, char)| {
    //         let num = index + 1;
    //         let count = char.iter().filter(|y| **y == 'O').count();
    //         num * count
    //     })
    //     .sum::<usize>()
    0
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
