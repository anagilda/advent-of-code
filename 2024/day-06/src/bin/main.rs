use std::collections::HashSet;
fn main(){
    println!("Hello, day 3!");
    
    // Read input file
    let input = include_str!("../../input.txt");
    // dbg!(input);

    // Complete part 1
    let output_part_1 = part1(input);
    dbg!(output_part_1);
    // 174103751

    // Complete part 2
    let output_part_2 = part2(input);
    dbg!(output_part_2);

}

fn parse_input(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_string()).collect())
        .collect()
}

fn find_guard_position(grid: &[Vec<String>]) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell == "^" {
                return Some((i, j));
            }
        }
    }
    None
}

fn follow_patrol_protocol(grid: &[Vec<String>], start: (usize, usize), direction: (isize, isize)) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    let mut current_position = start;
    let mut current_direction = direction;

    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    // Movement deltas for right turns (clockwise order: up, right, down, left)
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    
    loop {
        positions.push(current_position);

        // Compute the next position based on the current direction
        let (row, column) = current_position;
        let (drow, dcolumn) = current_direction;
        let next_position = (
            (row as isize + drow) as usize,
            (column as isize + dcolumn) as usize,
        );

        // Check if the next position is within bounds and "something" is in front
        if next_position.0 < rows as usize
            && next_position.1 < cols as usize
            && grid[next_position.0][next_position.1] == "#"
        {
            // Turn right 90 degrees (update the direction)
            let current_index = directions.iter().position(|&d| d == current_direction).unwrap();
            current_direction = directions[(current_index + 1) % directions.len()];
        } else {
            // Take a step forward
            if next_position.0 >= rows as usize || next_position.1 >= cols as usize {
                break; // Stop if moving out of bounds
            }
            current_position = next_position;
        }
    }

    positions
}


fn part1(_input: &str) -> i32 {
    let lab_map = parse_input(_input);
    println!("{:?}", lab_map);

    let initial_guard_position = find_guard_position(&lab_map).expect("Guard not found in the lab map");
    println!("Guard found at position: {:?}", initial_guard_position);
    
    // Start at the guard position and follow the patrol protocol going up (^)
    let path = follow_patrol_protocol(&lab_map, initial_guard_position, (-1, 0));
    let unique_positions: HashSet<_> = path.clone().into_iter().collect();
    // todo: create a visualization?

    dbg!(&path);

    unique_positions.len() as i32
}




fn part2(_input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part1_example() {
        let result = part1("....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...");
        let expected = 41;
        assert_eq!(result, expected);
    }
}
