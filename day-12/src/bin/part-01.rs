fn main() {
    let input = include_str!("./input.txt");
    let num = process(input);
    println!(" ");
    println!("num => {num}")
}

fn process(input: &str) -> usize {
    println!("input {input}");
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn works() {
        let input = include_str!("./test.txt");
        assert_eq!(1030, process(input));
    }
}
