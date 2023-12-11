use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let num = process(input);
    println!(" ");
    println!("num => {num}")
}

#[allow(dead_code)]
fn generate_pairs(n: isize) -> Vec<(isize, isize)> {
    let mut pairs = Vec::new();
    for i in 1..=n {
        let min = n;
        let ma = i + 1;
        for j in ma..=min {
            pairs.push((i, j));
        }
    }
    pairs
}

fn process(input: &str) -> usize {
    let mut lines = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .map(|x| x.iter().map(|c| c.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut columns = vec![];
    let mut rows = vec![];
    let mut galaxy_count = 0_isize;

    for x in 0..lines.len() {
        {
            let val = &lines[x];
            if val.iter().all(|x| x == ".") {
                columns.push(x);
            }
        }
        {
            let val = &lines[x];
            for (y, _) in val.iter().enumerate() {
                if x == y {
                    let mut row_array = vec![];
                    for (ind, _) in lines.iter().enumerate() {
                        let vert = lines[ind][y].clone();
                        row_array.push(vert)
                    }
                    if row_array.iter().all(|ind| ind == ".") {
                        rows.push(x);
                    }
                }
            }
        }
        {
            let len = lines[x].len();
            for y in 0..len {
                let val = &lines[x][y];
                if val == "#" {
                    galaxy_count += 1;
                    let i = galaxy_count.to_string();
                    lines[x][y] = i;
                }
            }
        }
    }

    // println!("columns=> {columns:?}  row=> {rows:?}");

    for line in lines.iter_mut() {
        {
            for y in 0..line.len() {
                for c in &rows {
                    if y == *c {
                        line[y] = "x".to_string();
                    }
                }
            }
        }
    }

    for x in 0..lines.len() {
        let len = lines[x].len();
        for c in &columns {
            if x == *c {
                for i in 0..len {
                    lines[x][i] = "x".to_string();
                }
            }
        }
    }

    // for line in &lines {
    //     println!("line => {:?}", line);
    // }

    let mut galaxy_location = HashMap::new();

    for (x, line) in lines.iter().enumerate() {
        for (y, val) in line.iter().enumerate() {
            if val != "." {
                let x = x as isize;
                let y = y as isize;
                galaxy_location.insert(val.clone(), (x, y));
            }
        }
    }

    let pairs = generate_pairs(galaxy_count);
    let rows = rows;

    pairs
        .iter()
        .map(|pair| {
            let x = pair.0.to_string();
            let y = pair.1.to_string();
            let loc_x = galaxy_location.get(&x).expect("msg");
            let loc_y = galaxy_location.get(&y).expect("msg");

            let mut for_x_axis = 0;
            let mut for_x = [loc_x.0, loc_y.0];
            let mut for_y = [loc_x.1, loc_y.1];
            for_x.sort();
            for_y.sort();

            let growth = 1000000 - 1;

            for x in for_x[0]..for_x[1] {
                let x = x as usize;
                for y in &columns {
                    if y == &x {
                        for_x_axis += growth;
                    }
                }
            }

            let mut for_y_axis = 0;
            for x in for_y[0]..for_y[1] {
                let x = x as usize;
                for y in &rows {
                    if y == &x {
                        for_y_axis += growth;
                    }
                }
            }

            let count = for_y_axis + for_x_axis;
            // println!("for_y_axis ={for_y_axis}  for_x_axis {for_x_axis}");
            // }

            let x_axis = loc_x.0.abs_diff(loc_y.0);
            let y_axis = loc_x.1.abs_diff(loc_y.1);

            x_axis + y_axis + count
        })
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn universe_is_10_times() {
        let input = include_str!("./test.txt");
        assert_eq!(1030, process(input));
    }

    #[test]
    fn universe_is_100_times() {
        let input = include_str!("./test.txt");
        assert_eq!(8410, process(input));
    }
}
