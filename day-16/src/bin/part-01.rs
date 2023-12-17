fn main() {
    let input = include_str!("./test.txt");
    println!("ans {}", process(input));
}

fn process(input: &str) -> usize {
    let lines = input
        .lines()
        .enumerate()
        .map(|(x, val)| {
            val.chars()
                .enumerate()
                .map(|(y, char)| Tile::new(char, x, y))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let pos = lines[0][0].pos;
    let mut lines = lines;

    run(pos, &mut lines);

    for x in lines {
        println!("{x:?}");
        println!(" ");
    }
    0
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone, Copy)]
struct Tile {
    energized: bool,
    tile_type: TileType,
    pos: Pos,
    from: Direction, // to
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl Tile {
    fn new(char: char, x: usize, y: usize) -> Self {
        Self {
            energized: false,
            tile_type: char.into(),
            pos: Pos { x, y },
            ..Default::default()
        }
    }

    fn next(&mut self, lines: &mut [Vec<Tile>]) {}

    fn set_from(&mut self, to: Direction) {
        self.from = to;
    }

    fn energize(&mut self) {
        self.energized = true;
    }

    fn is_valid_next_move(&self, going: Direction, lines: &[Vec<Tile>]) -> bool {
        let current_pos = self.pos;
        let outer_length = lines.len();
        let inner_length = lines.first().expect("has").len();
        let at_extreme_left = (current_pos.y as isize - 1).is_negative();
        let at_extreme_right = (current_pos.y + 1) >= inner_length;
        let at_extreme_top = (current_pos.x as isize - 1).is_negative();
        let at_extreme_bottom = (current_pos.x + 1) >= outer_length;

        let can_go = |value: bool| !value;

        match going {
            Direction::Up => can_go(at_extreme_top),
            Direction::Down => can_go(at_extreme_bottom),
            Direction::Left => can_go(at_extreme_left),
            Direction::Right => can_go(at_extreme_right),
        }
        // match self
    }

    fn start_move(&mut self, lines: &mut [Vec<Tile>]) {
        let tile = self;
        match tile.tile_type {
            TileType::EmptySpace => todo!(),
            TileType::R90Mirror => todo!(),
            TileType::L90Mirror => todo!(),
            TileType::HorizontalSplitter => todo!(),
            TileType::VerticalSplitter => todo!(),
        }
    }

    // fn get_next_tile(&self, to: Direction, lines: &mut [Vec<Tile>]) -> Vec<Pos> {
    //     let pos = self.pos;

    //     match to {
    //         Direction::Up => {
    //             let x = pos.x - 1;
    //             let tile = lines[x][pos.y];
    //         }
    //         Direction::Down => {
    //             let x = pos.x + 1;
    //             lines[x][pos.y]
    //         }
    //         Direction::Left => {
    //             let y = pos.y - 1;
    //             lines[pos.x][y]
    //         }
    //         Direction::Right => {
    //             let y = pos.y + 1;
    //             lines[pos.x][y]
    //         }
    //     }
    // }

    // fn move_from_empty_space()
}

fn run(pos: Pos, lines: &mut [Vec<Tile>]) {
    let mut tile = lines[pos.x][pos.y];
    tile.energize();
    if tile.is_valid_next_move(tile.from, lines) {
        return;
    }
    // let mut new_tile = tile.get_next_tile(tile.from, lines);
    // new_tile.set_from(tile.from);
    //get next tile

    tile.start_move(lines);
}

impl From<char> for TileType {
    fn from(val: char) -> Self {
        match val {
            '.' => TileType::EmptySpace,
            '/' => TileType::R90Mirror,
            '\\' => TileType::L90Mirror,
            '|' => TileType::VerticalSplitter,
            '-' => TileType::HorizontalSplitter,
            _ => unreachable!(" not here"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone, Copy)]
enum TileType {
    #[default]
    EmptySpace,
    R90Mirror,
    L90Mirror,
    HorizontalSplitter,
    VerticalSplitter,
}

#[cfg(test)]
mod test {
    use crate::process;

    #[test]
    fn works() {
        let input = include_str!("./test.txt");
        assert_eq!(46, process(input))
    }
}
