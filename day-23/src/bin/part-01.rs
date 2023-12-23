use std::collections::HashMap;

fn main() {
    let input = include_str!("./test.txt");
    println!("{}", process(input));
}

#[allow(dead_code)]
#[derive(Debug, Default, Hash, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Tile {
    // pos: Pos,
    tile_type: Type,
    neighbor: Vec<Pos>,
}

impl Tile {
    fn new(tile_type: Type, neighbor: Vec<Pos>) -> Self {
        Self {
            // pos,
            tile_type,
            neighbor,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Type {
    Path,
    Slope(Direction),
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
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut starting_pos = Pos {
        ..Default::default()
    };

    let mut map_hash = HashMap::new();
    let x_length = lines.len();

    for (x, line) in lines.iter().enumerate() {
        let y_lenght = line.len();
        for (y, char) in line.iter().enumerate() {
            if x == 0 && char == &'.' {
                starting_pos = Pos { x, y };
            }

            let tile_type_option = match char {
                '.' => Some(Type::Path),
                '>' => Some(Type::Slope(Direction::Right)),
                '<' => Some(Type::Slope(Direction::Left)),
                'v' => Some(Type::Slope(Direction::Down)),
                '^' => Some(Type::Slope(Direction::Up)),
                _ => None,
            };

            if let Some(tile_type) = tile_type_option {
                let pos = Pos { x, y };
                let neighbor = get_neigbor(&pos, x_length, y_lenght, &lines);
                let tile = Tile::new(tile_type, neighbor);
                map_hash.insert(pos, tile);
            }
        }
    }

    for x in map_hash {
        if x.0.x == 0 {
            println!("{x:?}");
        }
    }
    println!(" ");

    println!("starting pos {:?}", starting_pos);
    // println!("input: {input}");
    0
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
