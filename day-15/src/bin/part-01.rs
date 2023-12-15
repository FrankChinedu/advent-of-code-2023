fn main() {
    let input = include_str!("./test.txt");
    println!("ans {}", process(input));
}

fn get_ascii(char: char) -> u32 {
    char.into()
}

fn process(input: &str) -> u32 {
    input
        .split(',')
        .map(|word| {
            word.chars().map(get_ascii).fold(0, |a, b| {
                let current_val = a + b;
                let current_val = current_val * 17;
                current_val % 256
            })
        })
        .sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_ascii_works() {
        assert_eq!(72, get_ascii('H'));
    }

    #[test]
    fn hash_hash_works() {
        assert_eq!(52, process("HASH"));
    }

    #[test]
    fn test_input_works() {
        let input = include_str!("./test.txt");
        assert_eq!(1320, process(input));
    }
}
