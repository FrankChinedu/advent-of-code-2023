use std::{collections::HashMap, usize};

fn main() {
    let x = 60 / 5 * (7 - 4);
    println!("x {x}");
    let input = include_str!("./test.txt");
    let num = process(input);
    println!("num:{num}");
}

const UP: Direction = Direction::Up { x: -1, y: 0 };
const DOWN: Direction = Direction::Down { x: 1, y: 0 };
const LEFT: Direction = Direction::Left { x: 0, y: -1 };
const RIGHT: Direction = Direction::Right { x: 0, y: 1 };

fn process(input: &str) -> usize {
    let pipes = vec![
        ('|', vec![UP, DOWN]),
        ('-', vec![RIGHT, LEFT]),
        ('L', vec![UP, RIGHT]),
        ('J', vec![UP, LEFT]),
        ('7', vec![DOWN, LEFT]),
        ('F', vec![DOWN, RIGHT]),
        ('.', vec![]),
        ('S', vec![LEFT, UP, RIGHT, DOWN]),
    ];

    let map = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| Pipe::new(c, pipes.clone()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut position_of_s = [0, 0];

    let mut graph_map = HashMap::new();

    for (x, pipes) in map.iter().enumerate() {
        for (y, pipe) in pipes.iter().enumerate() {
            if pipe.symbol == 'S' {
                position_of_s = [x, y];
            }
            let pos = (x, y);
            graph_map.insert(pos, false);
        }
    }

    let dimentions = [
        map.len(),
        map.iter().map(|c| c.len()).sum::<usize>() / map.len(),
    ];

    // for (key, val) in graph_map {
    //     println!("key: {key:?} => val {val}");
    // }
    let x = position_of_s[0];
    let y = position_of_s[1];
    let search_pipe: &Pipe = &map[x][y];
    // loop {
    // let mut x_dir = x;
    // let mut y_dir = y;

    fn find_destination(
        map: &[Vec<Pipe>],
        pipe: &Pipe,
        x: usize,
        y: usize,
        mut count: usize,
        from: &Direction,
    ) -> usize {
        let oppisite_of_from = from.get_opposite();
        let new_direction = pipe
            .directions
            .iter()
            .filter(|x| **x != oppisite_of_from)
            .collect::<Vec<_>>();
        let new_direction = *new_direction[0];
        print!(" {} => ", pipe.symbol);
        // println!("x=>{x} y=>{y}");
        // println!(
        //     "Pipe.symbol = {:?} new_direction ={:?}",
        //     pipe, new_direction
        // );

        count += 1;

        if pipe.symbol == 'S' {
            count
        } else {
            let new_pos = new_direction.get();
            let new_pipe = get_pipe(map, &new_pos, x, y);
            let new_x = (new_pos.x + x as isize) as usize;
            let new_y = (new_pos.y + y as isize) as usize;
            // println!("new_x=>{new_x} new_y=>{new_y}");
            // println!(
            //     "new_pipe.symbol = {:?} new_direction ={:?}",
            //     new_pipe, new_direction
            // );
            // println!("==============================================");
            find_destination(map, &new_pipe, new_x, new_y, count, &new_direction)
        }
    }

    let mut from = vec![];

    for direction in &search_pipe.directions {
        let is_valid = valid_move(direction, x, y, dimentions);
        if is_valid && get_pipe(&map, &direction.get(), x, y).symbol != '.' {
            let pipe = get_pipe(&map, &direction.get(), x, y);
            let position = direction.get();
            println!("x=>{x} y=>{x} ===>");
            println!(
                "Pipe.symbol = {:?} direction ={:?} position {position:?}",
                pipe, direction
            );
            let new_x = (x as isize + position.x) as usize;
            let new_y = (y as isize + position.y) as usize;
            println!("new_x=>{new_x} new_x=>{y} ===>");

            let step_count = 0;
            println!(" ");
            let val = find_destination(&map, &pipe, new_x, new_y, step_count, direction);
            // println!(" val ={val}");
            from.push((direction, val));
            println!("from: {:?}", from);

            // 's_loop: loop {

            // }
            // break val;
        } else {
            continue;
        }
    }
    // }

    println!("position_of_s:{position_of_s:?}");
    0
}

fn get_pipe(map: &[Vec<Pipe>], pos: &Pos, x: usize, y: usize) -> Pipe {
    let x = (x as isize + pos.x) as usize;
    let y = (y as isize + pos.y) as usize;
    map[x][y].clone()
}

fn valid_move(direction: &Direction, x: usize, y: usize, dimentions: [usize; 2]) -> bool {
    let direction = direction.get();
    let dir_x = dimentions[0] as isize;
    let dir_y = dimentions[1] as isize;
    let x = x as isize;
    let y = y as isize;
    let outer = direction.x + x;
    let inner = direction.y + y;

    if outer.is_negative() || inner.is_negative() || outer >= dir_x || inner >= dir_y {
        return false;
    }
    true
}

// enum Poles {
//     North,
//     South,
//     East,
//     West,
// }

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up { x: i8, y: i8 },
    Down { x: i8, y: i8 },
    Left { x: i8, y: i8 },
    Right { x: i8, y: i8 },
}

#[derive(Debug)]
struct Pos {
    x: isize,
    y: isize,
}

impl From<Pos> for Direction {
    fn from(pos: Pos) -> Self {
        //         UP: Direction = Direction::Up { x: -1, y: 0 };
        // const DOWN: Direction = Direction::Down { x: 1, y: 0 };
        // const LEFT: Direction = Direction::Left { x: 0, y: -1 };
        // const RIGHT: Direction = Direction::Right { x: 0, y: 1 };
        match pos {
            pos if pos.x == -1 && pos.y == 0 => Self::Up { x: -1, y: 0 },
            pos if pos.x == 1 && pos.y == 0 => Self::Down { x: 1, y: 0 },
            pos if pos.x == 0 && pos.y == -1 => Self::Left { x: 0, y: -1 },
            pos if pos.x == 0 && pos.y == 1 => Self::Right { x: 0, y: 1 },
            _ => unreachable!("should not get here"),
        }
    }
}

impl Direction {
    fn get(self) -> Pos {
        match self {
            Direction::Up { x, y } => Pos {
                x: x.into(),
                y: y.into(),
            },
            Direction::Down { x, y } => Pos {
                x: x.into(),
                y: y.into(),
            },
            Direction::Left { x, y } => Pos {
                x: x.into(),
                y: y.into(),
            },
            Direction::Right { x, y } => Pos {
                x: x.into(),
                y: y.into(),
            },
        }
    }

    fn get_opposite(self) -> Self {
        match self {
            Direction::Up { .. } => self::DOWN,
            Direction::Down { .. } => self::UP,
            Direction::Left { .. } => self::RIGHT,
            Direction::Right { .. } => self::LEFT,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Pipe {
    symbol: char,
    // connecting: (Poles, Poles),
    directions: Vec<Direction>,
}

impl Pipe {
    fn new(symbol: char, pipes: Vec<(char, Vec<Direction>)>) -> Self {
        let pipe = pipes.iter().filter(|x| x.0 == symbol).collect::<Vec<_>>();
        let pipe = pipe[0].clone();
        Self {
            symbol,
            directions: pipe.1,
        }
    }
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
