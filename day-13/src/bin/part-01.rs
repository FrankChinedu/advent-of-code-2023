fn main() {
    let input = include_str!("./test.txt");
    let num = process(input);
    println!("num ={num:?}");
}

fn get(lines: &Vec<&str>) -> (bool, usize) {
    let middle = lines.len().div_ceil(2);
    let (first, second) = lines.split_at(middle);

    let mut result = vec![];

    for (i, x) in first.iter().enumerate() {
        let diff = middle - (1 + i);
        match second.get(diff) {
            Some(y) => {
                result.push(y == x);
            }
            None => continue,
        }
    }
    let bool = result.iter().all(|x| x == &true);
    (bool, middle)
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
            let col = get(&lines);

            if col.0 {
                col.1 * 100
            } else {
                let lines = &lines;
                let lines = flip_array(lines);
                let lines = lines.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                let row = get(&lines);
                row.1
            }
        })
        .sum::<usize>()
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn works() {
        let input = include_str!("./test.txt");
        assert_eq!(405, process(input));
    }
}
