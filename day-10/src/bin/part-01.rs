fn main() {
    let input = include_str!("./test.txt");
    let num = process(input);
    println!("num:{num}");
}

fn process(input: &str) -> usize {
    println!("input:{input}");
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn process_works() {
        let input = include_str!("./test.txt");
        assert_eq!(114, process(input));
    }
}
