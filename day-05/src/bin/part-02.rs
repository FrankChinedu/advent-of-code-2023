use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn main() {
    let dir = std::env::current_dir().unwrap().display().to_string();
    let filename = format!("{dir}/src/bin/test.txt");
    // let mut new_str = String::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            println!("line={:?}", line)
        }
    }
    // let lines = include_str!("./test.txt").split(' ').collect::<Vec<_>>();
    // let _num = process(lines);
}

fn process(input: Vec<String>) -> usize {
    println!("input ={:?}", input);
    0
}
// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_1() {
//         // let input = read_file("test.txt").expect("unable to read file");
//         // assert_eq!(46, process(input))
//     }
// }
