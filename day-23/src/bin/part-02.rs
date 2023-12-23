use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", process(input));
}

#[allow(dead_code)]
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Tile {
    // pos: Pos,
    tile_type: Type,
    neighbor: Vec<Neighbor>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Neighbor {
    dir: Direction,
    pos: Pos,
}

impl Tile {
    fn new(tile_type: Type, neighbor: Vec<Neighbor>) -> Self {
        Self {
            // pos,
            tile_type,
            neighbor,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Type {
    Path,
    Slope(Direction),
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_neigbor(pos: &Pos, x_length: usize, y_lenght: usize, lines: &[Vec<char>]) -> Vec<Neighbor> {
    use Direction::*;
    let mut vec = vec![];

    let direction = vec![Up, Down, Left, Right];

    for dir in direction {
        if dir == Up && !(pos.x as isize - 1).is_negative() {
            let x = (pos.x as isize - 1) as usize;
            let pos = Pos { x, y: pos.y };
            let char = lines[pos.x][pos.y];
            if char != '#' {
                vec.push(Neighbor { dir: Up, pos })
            }
        }

        if dir == Down && (pos.x + 1) < x_length {
            let x = pos.x + 1;
            let pos = Pos { x, y: pos.y };
            let char = lines[pos.x][pos.y];
            if char != '#' {
                vec.push(Neighbor { dir: Down, pos })
            }
        }

        if dir == Left && !(pos.y as isize - 1).is_negative() {
            let y = (pos.y as isize - 1) as usize;
            let pos = Pos { x: pos.x, y };
            let char = lines[pos.x][pos.y];
            if char != '#' {
                vec.push(Neighbor { dir: Left, pos })
            }
        }

        if dir == Right && (pos.y + 1) < y_lenght {
            let y = pos.y + 1;
            let pos = Pos { x: pos.x, y };
            let char = lines[pos.x][pos.y];
            if char != '#' {
                vec.push(Neighbor { dir: Right, pos })
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

    let mut ending_pos = Pos {
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

            if x == x_length - 1 && char == &'.' {
                ending_pos = Pos { x, y };
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

    // println!("ending_pos {ending_pos:?}");
    // println!("starting_pos {starting_pos:?}");

    // for x in &map_hash {
    //     if x.0.x == 0 {
    //         println!("{x:?}");
    //     }
    // }
    // println!(" ");
    let mut count = 0u32;

    let mut clone_lines = lines.clone();
    let mut path_history = HashSet::new();

    // 'outer: loop {
    let mut history = HashSet::new();
    let mut history_state = vec![];
    let mut current_pos = starting_pos.clone();

    'travel: loop {
        if current_pos == ending_pos {
            break 'travel;
        }
        let current_pos_copy = current_pos.clone();
        if !history.contains(&current_pos) {
            history.insert(current_pos.clone());
            let char = &lines[current_pos.x][current_pos.y];
            let to_go = match char {
                '>' => Some(&Direction::Right),
                '<' => Some(&Direction::Left),
                'v' => Some(&Direction::Down),
                '^' => Some(&Direction::Up),
                _ => None,
            };
            let current_tile = map_hash.get(&current_pos).expect("has current tile");
            let directions = &current_tile.neighbor;
            if directions.len() > 2 {
                let his_state = HistoryState {
                    pos: current_pos.clone(),
                    count,
                    tile: current_tile.clone(),
                };
                history_state.push(his_state);
                let maps = directions
                    .iter()
                    .filter(|neighbor| !history.contains(&neighbor.pos))
                    .map(|x| x.pos.clone())
                    .collect::<Vec<_>>();

                path_history.extend(maps.clone());

                // break;
                // if history_state.len() > 1 {
                //     break;
                // }
            }

            for dir in directions {
                let char = clone_lines[dir.pos.x][dir.pos.y];
                if char == '.' {
                    clone_lines[dir.pos.x][dir.pos.y] = 'O';
                }

                let cur_pos = &dir.pos;
                if history.contains(cur_pos) {
                    continue;
                }

                if let Some(val) = to_go {
                    if val != &dir.dir {
                        continue;
                    }
                }

                count += 1;
                current_pos = cur_pos.clone();
            }
            if current_pos_copy == current_pos && !history_state.is_empty() {
                // break 'travel;
                let len: usize = history_state.len();
                let state = &history_state[len - 1];
                count = state.count;
                for state_neighbor in &state.tile.neighbor {
                    if !history.contains(&state_neighbor.pos) {
                        current_pos = state_neighbor.pos.clone();
                        break;
                    }
                }
            }
        }
    }
    // path_history.retain(|x| history.contains(x));
    // if path_history.is_empty() {
    //     break 'outer;
    // }
    // }

    // for x in path_history {
    //     println!("{x:?}");
    // }
    println!("count: {count}");
    0
}

#[allow(dead_code)]
#[derive(Debug)]
struct HistoryState {
    pos: Pos,
    count: u32,
    tile: Tile,
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
