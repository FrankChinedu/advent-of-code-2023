fn main() {
    let input = include_str!("./input1.txt");
    let sum = process(input);
    println!("answer {}", sum);
}

fn process(input: &str) -> u32 {
    input.lines().map(process_line).sum::<u32>()
}

fn process_line(line: &str) -> u32 {
    println!("line {:?}", line);
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];

        println!("reduced_line {:?}", reduced_line);
        let result = if reduced_line.starts_with("one") {
            '1'
        } else if reduced_line.starts_with("two") {
            '2'
        } else if reduced_line.starts_with("three") {
            '3'
        } else if reduced_line.starts_with("four") {
            '4'
        } else if reduced_line.starts_with("five") {
            '5'
        } else if reduced_line.starts_with("six") {
            '6'
        } else if reduced_line.starts_with("seven") {
            '7'
        } else if reduced_line.starts_with("eight") {
            '8'
        } else if reduced_line.starts_with("nine") {
            '9'
        } else {
            reduced_line.chars().next().unwrap()
        };

        println!("result --> {:?}", result);

        result.to_digit(10)
    });

    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("should be a valid number")
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
