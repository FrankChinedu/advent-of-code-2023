fn main() {
    let input = include_str!("./input.txt");
    println!("{}", process(input));
}

fn process(input: &str) -> usize {
    let snapshot = input.lines().collect::<Vec<_>>();

    let bricks = parse_input(snapshot);
    let grid = simulate_brick_positions(&bricks);
    let support_relationships = analyze_support_relationships(&bricks, &grid);
    let safe_disintegrations = count_safe_disintegrations(&bricks, &support_relationships);

    println!(
        "Number of bricks that can be safely disintegrated: {}",
        safe_disintegrations
    );
    0
}

use std::collections::{HashMap, HashSet};

// Step 1: Parse Input
fn parse_input(snapshot: Vec<&str>) -> Vec<((i32, i32, i32), (i32, i32, i32))> {
    snapshot
        .iter()
        .map(|line| {
            let coords: Vec<&str> = line.split('~').collect();
            let start: Vec<i32> = coords[0].split(',').map(|s| s.parse().unwrap()).collect();
            let end: Vec<i32> = coords[1].split(',').map(|s| s.parse().unwrap()).collect();
            ((start[0], start[1], start[2]), (end[0], end[1], end[2]))
        })
        .collect()
}

// Step 2: Simulate Brick Positions
fn simulate_brick_positions(
    bricks: &Vec<((i32, i32, i32), (i32, i32, i32))>,
) -> HashSet<(i32, i32, i32)> {
    let mut grid = HashSet::new();
    for brick in bricks {
        let (start, end) = brick;
        let (start_x, start_y, start_z) = start;
        let (end_x, end_y, end_z) = end;

        // Add all positions between start and end (inclusive) to the grid
        for x in *start_x..*end_x {
            for y in *start_y..*end_y {
                for z in *start_z..*end_z {
                    grid.insert((x, y, z));
                }
            }
        }
    }
    grid
}

// Step 3: Analyze Support Relationships
fn analyze_support_relationships(
    bricks: &Vec<((i32, i32, i32), (i32, i32, i32))>,
    grid: &HashSet<(i32, i32, i32)>,
) -> HashMap<(i32, i32, i32), Vec<(i32, i32, i32)>> {
    let mut support_relationships: HashMap<(i32, i32, i32), Vec<(i32, i32, i32)>> = HashMap::new();
    for brick in bricks {
        let (_, end) = brick;
        support_relationships
            .entry(*end)
            .or_insert(Vec::new())
            .push(brick.0);
    }
    support_relationships
}

// Step 4: Check Safety for Disintegration
fn is_safe_to_disintegrate(
    brick: &((i32, i32, i32), (i32, i32, i32)),
    support_relationships: &HashMap<(i32, i32, i32), Vec<(i32, i32, i32)>>,
) -> bool {
    let (_, end) = brick;
    let binding = vec![];
    let supported_bricks = support_relationships.get(end).unwrap_or(&binding);
    supported_bricks.iter().all(|supported_brick| {
        support_relationships
            .get(supported_brick)
            .unwrap_or(&vec![])
            .is_empty()
    })
}

// Step 5: Count Safe Disintegrations
fn count_safe_disintegrations(
    bricks: &Vec<((i32, i32, i32), (i32, i32, i32))>,
    support_relationships: &HashMap<(i32, i32, i32), Vec<(i32, i32, i32)>>,
) -> usize {
    bricks
        .iter()
        .filter(|brick| is_safe_to_disintegrate(brick, support_relationships))
        .count()
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
