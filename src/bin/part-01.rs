fn main() {
    let input = include_str!("./input1.txt");
    let sum = process(input);
    println!("answer {}", sum);
}

fn process(input: &str) -> i32 {
    let input_arr = input
        .lines()
        .map(|x| x.chars().filter(|x| x.is_numeric()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut b = vec![];

    for v in input_arr {
        let c: i32 = format!("{}{}", v.first().unwrap(), v.last().unwrap())
            .parse()
            .ok()
            .unwrap();
        b.push(c);
    }
    b.iter().sum::<i32>()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";
        assert_eq!(142, process(input))
    }
}
