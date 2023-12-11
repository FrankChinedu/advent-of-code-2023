use std::collections::HashMap;

fn main() {
    let c = -1 + 2;
    println!("c {c}");
    let input = include_str!("./test.txt");
    let num = process(input);
    println!(" ");
    println!("num => {num}")
}

use std::collections::VecDeque;

#[allow(dead_code)]
// Function to find the shortest distance between two elements in a matrix
fn shortest_distance(
    matrix: &[Vec<i32>],
    start: (usize, usize),
    end: (usize, usize),
) -> Option<usize> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    // Check if start and end positions are within the matrix
    if start.0 >= rows || start.1 >= cols || end.0 >= rows || end.1 >= cols {
        return None;
    }

    // Directions for moving to neighboring cells (up, down, left, right)
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // Create a queue for BFS
    let mut queue = VecDeque::new();
    queue.push_back((start, 0)); // Start position and distance

    // Visited array to track visited cells
    let mut visited = vec![vec![false; cols]; rows];
    visited[start.0][start.1] = true;

    while let Some(((r, c), distance)) = queue.pop_front() {
        if (r, c) == end {
            return Some(distance);
        }

        for (dr, dc) in &directions {
            let new_r = (r as i32 + dr) as usize;
            let new_c = (c as i32 + dc) as usize;

            // Check if the new position is within the matrix and hasn't been visited
            if new_r < rows && new_c < cols && !visited[new_r][new_c] {
                queue.push_back(((new_r, new_c), distance + 1));
                visited[new_r][new_c] = true;
            }
        }
    }

    // If no path is found
    None
}

#[allow(dead_code)]
fn factorial(n: isize) -> isize {
    if n == 0 || n == 1 {
        1
    } else {
        (2..=n).product()
    }
}

#[allow(dead_code)]
fn combinations(n: isize, k: isize) -> isize {
    factorial(n) / (factorial(k) * factorial(n - k))
}

fn generate_pairs(n: isize) -> Vec<(isize, isize)> {
    let mut pairs = Vec::new();

    for i in 1..=n {
        let min = n;
        let ma = i + 1;
        for j in ma..=min {
            pairs.push((i, j));
        }
    }

    println!("len {}", pairs.len());

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

    println!("columns {columns:?}");
    println!("rows {rows:?}");

    println!(" ");
    // let mut lines = lines;
    let len = lines.len();

    for line in lines.iter_mut() {
        {
            for y in 0..line.len() {
                for (id, c) in rows.iter().enumerate() {
                    if y == *c {
                        let el = c + id;
                        line.insert(el, ".".to_string());
                    }
                }
            }
        }
    }

    for x in 0..len {
        let len = lines[x].len();
        for (id, c) in columns.iter().enumerate() {
            let mut arr = vec![];
            arr.resize(len, ".".to_string());
            if x == *c {
                let el = c + id;
                lines.insert(el, arr);
            }
        }
    }

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

    pairs
        .iter()
        .map(|pair| {
            let x = pair.0.to_string();
            let y = pair.1.to_string();
            let loc_x = galaxy_location.get(&x).expect("msg");
            let loc_y = galaxy_location.get(&y).expect("msg");

            let x_axis = loc_x.0.abs_diff(loc_y.0);
            let y_axis = loc_x.1.abs_diff(loc_y.1);
            x_axis + y_axis
        })
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn process_works() {
        let input = include_str!("./test.txt");
        assert_eq!(374, process(input));
    }
}
