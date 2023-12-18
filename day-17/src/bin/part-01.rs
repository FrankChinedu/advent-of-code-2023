fn main() {
    let input = include_str!("./test.txt");
    println!("{}", process(input));
}

fn process(input: &str) -> usize {
    println!("input{input}");
    0
}

#[cfg(test)]
mod test {
    #[test]
    fn works() {}
}
