fn main() {
    let time = [61, 67, 75, 71].to_vec();
    let distance = [430, 1036, 1307, 1150].to_vec();

    let num = process(time, distance);

    println!("num={num}")
}

fn process(time: Vec<i32>, distance: Vec<i32>) -> usize {
    let hold = 1;
    let mut ways_to_win = vec![];
    for (index, val) in time.iter().enumerate() {
        let distance_to_comp = distance[index];
        let mut count = 0;
        for hold_val in hold..*val {
            let distance_traveled = hold_val * (val - hold_val);
            if distance_traveled > distance_to_comp {
                count += 1;
            }
        }
        ways_to_win.push(count);
    }
    ways_to_win.iter().product()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(288, process([7, 15, 30].to_vec(), [9, 40, 200].to_vec()))
    }
}
