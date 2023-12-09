fn main() {
    let input = include_str!("./input.txt");
    let num = process(input);
    println!("num:{num}");
}
fn calc_lagrange(arr: Vec<i128>) -> Vec<i128> {
    arr.iter()
        .enumerate()
        .map(|(index, value)| lagrange(arr.len() as i128, index as i128, *value))
        .collect::<Vec<_>>()
}

fn lagrange(arr_length: i128, index: i128, value: i128) -> i128 {
    let term = arr_length;

    let mut top_divisor = 1_i128;
    let mut bottom_divisor = 1_i128;

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

fn to_integer_array(input: &str) -> Vec<i128> {
    input
        .split(' ')
        .map(|a| a.parse::<i128>().expect("msg"))
        .collect::<Vec<_>>()
}

fn process(input: &str) -> i128 {
    input
        .lines()
        .map(to_integer_array)
        .map(|dataset| calc_lagrange(dataset).iter().sum::<i128>())
        .sum::<i128>()
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
        let ans = calc_lagrange(to_integer_array("0 3 6 9 12 15"))
            .iter()
            .sum::<i128>();
        assert_eq!(18, ans);
    }

    #[test]
    fn calc_lagrange_works_2() {
        let ans = calc_lagrange(to_integer_array("1 3 6 10 15 21"))
            .iter()
            .sum::<i128>();
        assert_eq!(28, ans);
    }

    #[test]
    fn calc_lagrange_works_3() {
        let ans = calc_lagrange(to_integer_array("10 13 16 21 30 45"))
            .iter()
            .sum::<i128>();
        assert_eq!(68, ans);
    }

    #[test]
    fn calc_lagrange_works_4() {
        let ans = calc_lagrange(
          to_integer_array("7 10 23 62 147 303 562 967 1585 2545 4127 6933 12171 22103 40826 75939 142603 273520 542191 1113540 2347099")
        ).iter().sum::<i128>();
        assert_eq!(4995427, ans); //
    }
}
