fn main() {
    let input = include_str!("./test.txt");
    let num = process(input);
    println!(" ");
    println!("num => {num}")
}

fn process(input: &str) -> usize {
    let mut lines = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .map(|x| x.iter().map(|c| c.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut columns = vec![];
    let mut rows = vec![];
    let mut galaxy_count = 0;

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

    // let mut lines = lines;
    let len = lines.len();

    for line in lines.iter_mut() {
        {
            for y in 0..line.len() {
                for c in &rows {
                    if y == *c {
                        line.insert(*c + 1, ".".to_string());
                    }
                }
            }
        }
    }

    for x in 0..len {
        let len = lines[x].len();
        for c in &columns {
            let mut arr = vec![];
            arr.resize(len, ".".to_string());
            if x == *c {
                lines.insert(*c, arr);
            }
        }
    }

    for line in lines {
        println!("line ={line:?}");
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn process_works() {
        let input = include_str!("./test.txt");
        assert_eq!(114, process(input));
    }
}
