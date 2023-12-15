fn main() {
    let input = include_str!("./test.txt");
    println!("ans {}", process(input));
}

fn get_ascii(char: char) -> u32 {
    char.into()
}

fn get_hash(word: &str) -> u32 {
    word.chars().map(get_ascii).fold(0, |a, b| {
        let current_val = a + b;
        let current_val = current_val * 17;
        current_val % 256
    })
}

#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
struct Lens<'a> {
    label: &'a str,
    sign: Sign,
    focal_length: u32,
    hash: u32,
}

impl<'a> Lens<'a> {
    fn new(label: &'a str, sign: Sign, focal_length: u32, hash: u32) -> Self {
        Self {
            label,
            sign,
            focal_length,
            hash,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
#[allow(dead_code)]
enum Sign {
    #[default]
    Equal,
    Dash,
}

fn process(input: &str) -> usize {
    let lenses = input
        .split(',')
        .map(|word| {
            let word = word
                .split(['=', '-'])
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>();

            match word[..] {
                [a] => Lens::new(a, Sign::Dash, 0, get_hash(a)),
                [a, b] => Lens::new(a, Sign::Equal, b.parse::<u32>().expect("msg"), get_hash(a)),
                _ => unreachable!("should not get here"),
            }
        })
        .collect::<Vec<_>>();

    // for x in &lenses {
    //     println!("{x:?}");
    // }

    println!(" ");

    let capacity = 4;
    // let capacity = 256;
    let mut boxes: Vec<Option<Vec<Lens<'_>>>> = Vec::with_capacity(capacity);

    for _ in 0..capacity {
        boxes.push(Default::default());
    }

    for lens in lenses {
        let hash = lens.hash as usize;

        match &mut boxes[hash] {
            Some(arr) => match arr.iter_mut().position(|x| x.label == lens.label) {
                Some(i) => {
                    if lens.sign == Sign::Dash {
                        arr.remove(i);
                    } else {
                        arr[i] = lens
                    }
                }
                None => {
                    if lens.sign == Sign::Dash {
                        continue;
                    } else {
                        arr.push(lens)
                    }
                }
            },
            None => {
                if lens.sign == Sign::Dash {
                    continue;
                }
                boxes[hash] = Some(vec![lens]);
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let i = i + 1;
            let not_empty = item.clone().filter(|x| !x.is_empty()).is_some();
            if item.is_some() && not_empty {
                Some((i, item.clone().unwrap()))
            } else {
                None
            }
        })
        .filter_map(|x| x.clone())
        .flat_map(|(index, vec)| {
            vec.iter()
                .enumerate()
                .map(|(x, lens)| {
                    let x = x + 1;
                    index * x * lens.focal_length as usize
                })
                .collect::<Vec<_>>()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_works() {
        let input = include_str!("./test.txt");
        assert_eq!(145, process(input));
    }
}
