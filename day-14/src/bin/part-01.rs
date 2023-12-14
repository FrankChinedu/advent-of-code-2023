fn main() {
    let input = include_str!("./test.txt");
    println!("ans {}", process(input));
}

fn process(input: &str) -> usize {
    println!("input: {input}");
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("./test.txt");
        assert_eq!(20, process(input));
    }
}
