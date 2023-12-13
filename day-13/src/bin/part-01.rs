fn main() {
    let input = include_str!("./test.txt");
    let num = process(input);
    println!("num ={num:?}");
}

fn process(input: &str) -> usize {
    println!("input: {input:?}");
    0
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn universe_is_10_times() {
        let input = include_str!("./test.txt");
        assert_eq!(1030, process(input));
    }
}
