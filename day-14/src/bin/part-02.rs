fn main() {
    let input = include_str!("./test.txt");
    println!("ans {}", process(input));
}

enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn tilt_by_dir(&self, lines: &mut [Vec<char>], rock_locations: &Vec<(usize, usize)>) {
        // match self {
        //     Direction::North => (-1, 0),
        //     Direction::West => (0, -1),
        //     Direction::South => (1, 0),
        //     Direction::East => (0, 1),
        // }
        match self {
            Direction::North => self.tilt_north(lines, rock_locations),
            Direction::West => self.tilt_west(lines, rock_locations),
            _ => panic!(" should not reach"),
        }
    }

    fn run(&self, lines: &mut [Vec<char>]) -> usize {
        let rock_locations = self.get_rock_locations(lines);
        self.tilt_by_dir(lines, &rock_locations);
        self.sum(lines)
    }

    fn sum(&self, lines: &mut [Vec<char>]) -> usize {
        lines
            .iter_mut()
            .rev()
            .enumerate()
            .map(|(index, char)| {
                let num = index + 1;
                let count = char.iter().filter(|y| **y == 'O').count();
                num * count
            })
            .sum::<usize>()
    }

    fn get_rock_locations(&self, lines: &[Vec<char>]) -> Vec<(usize, usize)> {
        let mut rock_locations = vec![];
        for (x, line) in lines.iter().enumerate() {
            for (y, val) in line.iter().enumerate() {
                if val == &'O' {
                    let pos = (x, y);
                    rock_locations.push(pos);
                }
            }
        }
        rock_locations
    }

    fn tilt_north(&self, lines: &mut [Vec<char>], rock_locations: &Vec<(usize, usize)>) {
        for pos in rock_locations {
            let y = pos.1;
            for x in (1..=pos.0).rev() {
                let up_index = x - 1;
                let tmp_char = lines[up_index][y];
                let current_char = lines[x][y];
                if tmp_char == '.' && current_char == 'O' {
                    lines[up_index][y] = current_char;
                    lines[x][y] = tmp_char;
                }
            }
        }
    }

    fn tilt_west(&self, lines: &mut [Vec<char>], rock_locations: &Vec<(usize, usize)>) {
        for pos in rock_locations {
            // left
            let x = pos.0;
            for y in (1..=pos.1).rev() {
                let left_index = y - 1;
                let tmp_char = lines[x][left_index];
                let current_char = lines[x][y];
                if tmp_char == '.' && current_char == 'O' {
                    lines[x][y] = tmp_char;
                    lines[x][left_index] = current_char;
                }
            }
        }
        println!(" ");
    }
}

fn process(input: &str) -> usize {
    let _directions = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];
    let directions = [Direction::North, Direction::West];
    let lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    for x in &lines {
        println!("{x:?}")
    }

    println!(" ");

    let mut lines = lines;
    let mut sum = 0;

    for dir in directions {
        sum = dir.run(&mut lines);
        for x in &lines {
            println!("{x:?}")
        }

        println!(" ");
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("./test.txt");
        assert_eq!(136, process(input));
    }
}
