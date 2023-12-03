fn main() {
    let input = include_str!("./input.txt");
    let num = process(input);
    println!("num={num}")
}

#[allow(dead_code)]
struct Direction<'a> {
    up: (isize, isize, &'a str),
    down: (isize, isize, &'a str),
    left: (isize, isize, &'a str),
    up_left: (isize, isize, &'a str),
    up_right: (isize, isize, &'a str),
    down_left: (isize, isize, &'a str),
    down_right: (isize, isize, &'a str),
    right: (isize, isize, &'a str),
}

struct DirectionIter<'a, 'b> {
    direction: &'a Direction<'b>,
    position: u8,
}

impl<'a, 'b> Iterator for DirectionIter<'a, 'b> {
    type Item = (isize, isize, &'b str);

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.position {
            0 => Some(self.direction.up),
            1 => Some(self.direction.down),
            2 => Some(self.direction.left),
            3 => Some(self.direction.right),
            4 => Some(self.direction.up_left),
            5 => Some(self.direction.up_right),
            6 => Some(self.direction.down_left),
            7 => Some(self.direction.down_right),
            _ => None,
        };
        self.position += 1;
        next
    }
}

impl<'a, 'b> IntoIterator for &'a Direction<'b> {
    type Item = (isize, isize, &'b str);

    type IntoIter = DirectionIter<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            direction: self,
            position: 0,
        }
    }
}

impl<'a> Default for Direction<'a> {
    fn default() -> Self {
        Self {
            up: (-1, 0, "up"),
            down: (-1, 1, "down"),
            left: (0, -1, "left"),
            right: (0, 1, "right"),
            up_left: (-1, -1, "up_left"),
            up_right: (-1, 1, "up_right"),
            down_left: (1, 0, "down_left"),
            down_right: (1, 1, "down_right"),
        }
    }
}

fn process(input: &str) -> i32 {
    // let a = ' ';
    // println!(" is puncation a={}", a.is_alphanumeric());
    let directions = [
        (-1, 0, "up"),
        (-1, 1, "down"),
        (0, -1, "left"),
        (0, 1, "right"),
        (-1, -1, "up_left"),
        (-1, 1, "up_right"),
        (1, 0, "down_left"),
        (1, 1, "down_right"),
    ];
    let lines = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| {
                    if x.is_ascii_punctuation() && x != '.' || x.is_numeric() {
                        x
                    } else {
                        ' '
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let directions = Direction::default();

    for (i, line) in lines.iter().enumerate() {
        println!(" ");
        println!("outer line {:?}", line);
        let mut num_group_vec = vec![];
        let mut num_group = "".to_string();
        for (j, inner_line) in line.iter().enumerate() {
            println!("innerLine=*{inner_line}* len={}", line.len());
            let mut saw_synbol = false;

            for dir in &directions {
                // let is_last =
                let outer = i as isize + dir.0;
                let inner = j as isize + dir.1;
                let inner_line_length = line.len() as isize;
                let outter_line_length = lines.len() as isize;

                if outer.is_negative()
                    || inner.is_negative()
                    || inner > inner_line_length - 1
                    || outer > outter_line_length - 1
                {
                    continue;
                }

                let neighbor = lines[outer as usize][inner as usize];
                print!("neighbor=*{neighbor}*=>direction={} ", dir.2);
                if dir.2 == "right" {
                    if neighbor == ' ' || neighbor.is_ascii_punctuation() {
                        num_group = format!("{num_group}{inner_line}");
                        if neighbor.is_ascii_punctuation() {
                            saw_synbol = true;
                        }
                        println!("num_group {:?}", num_group);
                        if saw_synbol {
                            num_group_vec.push(num_group.to_string());
                            num_group = "".to_owned();
                        } else {
                            num_group = "".to_owned();
                        }
                    } else {
                        // num_group = format!("{num_group}{inner_line}");
                        num_group = "".to_owned();
                    }
                }
                if neighbor.is_ascii_punctuation() {
                    saw_synbol = true;
                }
            }
            print!("\n=============\n");
            // println!("num_group=>{:?}", num_group);
        }
        println!("num_group_vec=>{:?}", num_group_vec);
        if i == 0 {
            break;
        }
    }
    // println!("input: {lines:?}");
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        let input = include_str!("./input.txt");
        assert_eq!(4361, process(input))
    }
}
