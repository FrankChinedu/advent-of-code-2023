fn main() {
    let input = include_str!("./test.txt");
    let num = process(input);
    println!("num:{num}");
}
fn calc_lagrange(arr: Vec<isize>) -> Vec<isize> {
    arr.iter()
        .enumerate()
        .map(|(index, value)| lagrange(arr.len() as isize, index as isize, *value))
        .collect::<Vec<_>>()
}

fn lagrange(arr_length: isize, index: isize, value: isize) -> isize {
    let term = arr_length;

    let mut top_divisor = 1_isize;
    let mut bottom_divisor = 1_isize;

    for i in 0..arr_length {
        if i == index {
            continue;
        }
        let x = term - (i);
        top_divisor *= x;
        let y = index - (i);
        bottom_divisor *= y;
    }

    value * (top_divisor / bottom_divisor)
}

fn process(input: &str) -> isize {
    input
        .lines()
        .map(|x| {
            x.split(' ')
                .map(|a| a.parse::<isize>().expect("msg"))
                .collect::<Vec<_>>()
        })
        .map(|dataset| calc_lagrange(dataset).iter().sum::<isize>())
        .sum::<isize>()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn process_works() {
        let input = include_str!("./test.txt");
        assert_eq!(114, process(input));
    }

    #[test]
    fn calc_lagrange_works_1() {
        let arr = vec![0, 3, 6, 9, 12, 15];
        let ans = calc_lagrange(arr).iter().sum::<isize>();
        assert_eq!(18, ans);
    }

    #[test]
    fn calc_lagrange_works_2() {
        let arr = vec![1, 3, 6, 10, 15, 21];
        let ans = calc_lagrange(arr).iter().sum::<isize>();
        assert_eq!(28, ans);
    }

    #[test]
    fn calc_lagrange_works_3() {
        let arr = vec![10, 13, 16, 21, 30, 45];
        let ans = calc_lagrange(arr).iter().sum::<isize>();
        assert_eq!(68, ans);
    }
}
