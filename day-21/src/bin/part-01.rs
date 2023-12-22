use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", process(input));
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct Garden {
    part: Part,
    neihbors: Vec<Pos>,
}

impl Garden {
    fn new(part: Part, neihbors: Vec<Pos>) -> Self {
        Self { part, neihbors }
    }
}

#[allow(dead_code)]
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
enum Part {
    #[default]
    Rock,
    Plot,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_neigbor(pos: &Pos, x_length: usize, y_lenght: usize, lines: &[Vec<char>]) -> Vec<Pos> {
    use Direction::*;
    let mut vec = vec![];

    let direction = vec![Up, Down, Left, Right];

    for dir in direction {
        if dir == Up && !(pos.x as isize - 1).is_negative() {
            let x = (pos.x as isize - 1) as usize;
            let pos = Pos { x, y: pos.y };
            let char = lines[pos.x][pos.y];
            if char != '#' {
                vec.push(pos)
            }
        }

        if dir == Down && (pos.x + 1) < x_length {
            let x = pos.x + 1;
            let pos = Pos { x, y: pos.y };
            let char = lines[pos.x][pos.y];
            if char != '#' {
                vec.push(pos)
            }
        }

        if dir == Left && !(pos.y as isize - 1).is_negative() {
            let y = (pos.y as isize - 1) as usize;
            let pos = Pos { x: pos.x, y };
            let char = lines[pos.x][pos.y];
            if char != '#' {
                vec.push(pos)
            }
        }

        if dir == Right && (pos.y + 1) < y_lenght {
            let y = pos.y + 1;
            let pos = Pos { x: pos.x, y };
            let char = lines[pos.x][pos.y];
            if char != '#' {
                vec.push(pos)
            }
        }
    }
    vec
}

fn process(input: &str) -> usize {
    let mut garden_map = HashMap::new();
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let x_length = lines.len();

    let mut starting_pos = Pos {
        ..Default::default()
    };

    for (x, vec) in lines.iter().enumerate() {
        let y_lenght = vec.len();
        for (y, char) in vec.iter().enumerate() {
            if char == &'.' || char == &'S' {
                let part = Part::Plot;
                let pos = Pos { x, y };
                let neigbors: Vec<Pos> = get_neigbor(&pos, x_length, y_lenght, &lines);
                let sec = Garden::new(part, neigbors);
                if char == &'S' {
                    starting_pos = pos.clone();
                }
                garden_map.insert(pos, sec);
            }
        }
    }

    let starting_plot = garden_map.get(&starting_pos).expect("has starting pos");

    let num_of_steps = 64;

    let mut steps = num_of_steps - 1;

    let mut garden_plots = vec![];
    garden_plots.extend(starting_plot.neihbors.clone());

    // let mut cloned_lines = lines.clone();

    loop {
        // let mut inner_cloned_lines = lines.clone();
        let garden_plots_reached = garden_plots.len();
        if steps == 0 {
            break;
        }

        for _plot in 0..garden_plots_reached {
            let plot = garden_plots.remove(0);
            let garden_plot = garden_map.get(&plot).expect("has plot");
            // println!(" ");
            let neihbors = &garden_plot.neihbors;

            // for pos in neihbors {
            //     inner_cloned_lines[pos.x][pos.y] = 'O';
            // }
            garden_plots.extend(neihbors.clone());
        }

        // cloned_lines = inner_cloned_lines;

        steps -= 1;
    }

    let set: HashSet<Pos> = garden_plots.into_iter().collect();
    let garden_plots: Vec<Pos> = set.into_iter().collect();

    // for x in &garden_plots {
    //     println!("-> {x:?}");
    // }

    // println!(" ");
    // println!("num_of_steps {num_of_steps}");
    // println!(" ");

    // for line in &cloned_lines {
    //     println!(" {line:?}");
    // }

    garden_plots.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works() {
        let input = include_str!("./test.txt");
        assert_eq!(20, process(input));
    }
}
