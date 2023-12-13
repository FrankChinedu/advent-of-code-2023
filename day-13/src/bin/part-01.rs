fn main() {
    let input = include_str!("./test.txt");

    let num = process(input);
    println!("num ={num:?}");
}

fn generate_pairs(n: usize) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for i in 1..=n {
        let min = n;
        let ma = i + 1;
        for j in ma..=min {
            pairs.push((i, j));
        }
    }
    pairs
}

fn get(lines: &Vec<&str>) -> Option<usize> {
    let pairs = generate_pairs(lines.len());

    let mut pair: Option<(usize, usize)> = None;

    for current_pair in &pairs {
        let first = lines[current_pair.0 - 1];
        let second = lines[current_pair.1 - 1];
        if first == second {
            pair = Some(*current_pair);
            break;
        }
    }

    match pair {
        Some(pair_item) => {
            println!("pair_item ={pair_item:?} ");
            Some(pair_item.1.div_ceil(2))
        }
        None => None,
    }

    // let middle = lines.len().div_ceil(2);
    // let (first, second) = lines.split_at(middle);

    // let mut result = vec![];

    // for (i, x) in first.iter().enumerate() {
    //     let diff = middle - (1 + i);
    //     match second.get(diff) {
    //         Some(y) => {
    //             result.push(y == x);
    //         }
    //         None => continue,
    //     }
    // }
    // println!("\nresult ={result:?}");
    // println!("  ");
    // let bool = result.iter().all(|x| x == &true);
    // (bool, middle)
}

fn flip_array(lines: &[&str]) -> Vec<String> {
    let lines = lines
        .iter()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut vec: Vec<Vec<char>> = vec![];

    for (_, item1) in lines.iter().enumerate() {
        for (y, val) in item1.iter().enumerate() {
            match vec.get_mut(y) {
                Some(el) => el.push(*val),
                None => vec.push(vec![*val]),
            }
        }
    }

    let vec = vec
        .iter()
        .map(|x| x.iter().collect::<String>())
        .collect::<Vec<_>>();
    // let vec = vec.iter().map(|x| x.as_str()).collect::<Vec<_>>();
    vec
}

fn process(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|line| {
            let lines = line.lines().collect::<Vec<_>>();
            let new_lines = &lines;
            let new_lines = flip_array(new_lines);
            let new_lines = new_lines.iter().map(|x| x.as_str()).collect::<Vec<_>>();
            match get(&new_lines) {
                Some(val) => val * 100,
                None => get(&lines).expect("msg now"),
            }

            // if col.0 {
            // col.1
            // } else {
            //     let lines = &lines;
            //     let lines = flip_array(lines);
            //     let lines = lines.iter().map(|x| x.as_str()).collect::<Vec<_>>();
            //     let row = get(&lines);
            //     row.1
            // }
        })
        .sum::<usize>()
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn works() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
        // let input = include_str!("./test.txt");
        assert_eq!(405, process(input));
    }

    #[test]
    fn test_if_work() {
        let input = r#"...#.###.######
..##.##.##.##.#
####.#....#..#.
.#...#.#.#.##.#
.#..#.#.#######
##..#.#.#######
.#...#.#.#.##.#"#;

        assert_eq!(405, process(input));
    }
}
