fn main() {
    let input = include_str!("./input1.txt");

    let sum = process(input);
    println!("answer {}", sum);
}

fn process(input: &str) -> i32 {
    let valid_num_str = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut input_arr = input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();

    let to_num = |value| match value {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => unreachable!(),
    };

    for input in input_arr.iter_mut() {
        let mut order_of_occurence = vec![];
        for num_str in valid_num_str {
            if input.contains(num_str) {
                let pos = input
                    .match_indices(num_str)
                    .map(|(pos, _)| pos)
                    .collect::<Vec<_>>();
                for index in pos {
                    order_of_occurence.push((num_str, index));
                }
            }
        }

        order_of_occurence.sort_by(|a, b| a.1.cmp(&b.1));

        for (i, (a, _b)) in order_of_occurence.iter().enumerate() {
            if i == 0 || i == order_of_occurence.len() - 1 {
                *input = input.replace(a, to_num(a)).to_owned();
            }
        }
    }

    let input_arr: Vec<Vec<char>> = input_arr
        .iter()
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
        let input= "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(281, process(input))
    }

    #[test]
    fn test_2() {
        let input = r#"nqninenmvnpsz874
8twofpmpxkvvdnpdnlpkhseven4ncgkb
six8shdkdcdgseven8xczqrnnmthreecckfive
qlcnz54dd75nine7jfnlfgz
7vrdhggdkqbnltlgpkkvsdxn2mfpghkntzrhtjgtxr
cdhmktwo6kjqbprvfour8
ninekkvkeight9three
ms9five71lrfpqxqlbj
9five9sevenldshqfgcnq
1one4seven
7qtg3jmnd1two2
2c7four8one
qdkneight6zqcrgxxnkjdbb
twofourjqnpv4lgvzjzgnn"#;
        assert_eq!(901, process(input))
    }

    #[test]
    fn test_3() {
        let input = r#"3hlgrdsdsplnhpc
1vbdxqnzrthree
seven93six25sixnine
four1bxstchgzjdpxdninedpfour4
65fivetnsseight8lcvgkkglslcjtjssxmgtvk
fourjqgvkdkl2pxseven2ninemzqfqv
zqmsbltpvsrzcpnn2twolzdjqmb88
3qnzkmbltldthreesix1ffive36
kspfzvvvfkztcs9threefoureightsixseveneight"#;
        assert_eq!(448, process(input))
    }

    #[test]
    fn test_4() {
        let input = r#"oneightsevenine"#;
        assert_eq!(19, process(input))
    }

    #[test]
    fn test_5() {
        let input = r#"kspfzvvvfkztcs9threefoureightsixseveneight"#;
        assert_eq!(98, process(input))
    }

    #[test]
    fn test_6() {
        let input = r#"sevenineightseven"#;
        assert_eq!(77, process(input))
    }

    #[test]
    fn test_7() {
        let input = r#"eightwothree"#;
        assert_eq!(83, process(input))
    }
}
