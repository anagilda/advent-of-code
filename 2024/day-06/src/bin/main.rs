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
    // println!("{:?}", lab_map);

    let initial_guard_position = find_guard_position(&lab_map).expect("Guard not found in the lab map");
    // println!("Guard found at position: {:?}", initial_guard_position);
    
    // Start at the guard position and follow the patrol protocol going up (^)
    let path = follow_patrol_protocol(&lab_map, initial_guard_position, (-1, 0));
    let unique_positions: HashSet<_> = path.clone().into_iter().collect();
    // todo: create a visualization?

    dbg!(&path);

    unique_positions.len() as i32
}


fn simulate_patrol_protocol_with_obstructions(
    lab_map: &Vec<Vec<String>>,
    initial_position: (usize, usize),
    initial_direction: (isize, isize),
) -> Vec<(usize, usize)> {
    // Movement deltas for right turns (clockwise order: up, right, down, left)
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut positions_for_obstructions = Vec::new();

    let rows = lab_map.len();
    let columns = lab_map[0].len();

    // Iterate over every position in the lab map
    for row in 0..rows {
        for column in 0..columns {
            // Check if the position is free
            if lab_map[row][column] == "." {
                // Clone the lab map and add an obstruction "O" at the current position
                let mut lab_map_with_obstruction = lab_map.clone();
                lab_map_with_obstruction[row][column] = "O".to_string();
                
                // dbg!(&lab_map_with_obstruction);

                // Simulate patrol
                let mut current_position = initial_position;
                let mut current_direction = initial_direction;
                let mut visited_states = HashSet::new();

                loop {
                    // dbg!(&current_position);

                    // Record the current state (position and direction)
                    let state = (current_position, current_direction);
                    if visited_states.contains(&state) {
                        // If this state is revisited, break the loop
                        // for row in &lab_map_with_obstruction {
                        //     let row_str = row.join(""); // Join each string in the row into a single string
                        //     println!("{}", row_str);   // Print the row as a string
                        // }
                        // println!("{}", "-------------------");

                        positions_for_obstructions.push((row, column));
                        break;
                    }
                    visited_states.insert(state);

                    let (current_row, current_column) = current_position;
                    let (d_row, d_column) = current_direction;

                    // Calculate the next position
                    let next_position = (
                        current_row as isize + d_row,
                        current_column as isize + d_column,
                    );

                    // Check bounds and whether the position is obstructed
                    if next_position.0 >= 0
                        && next_position.1 >= 0
                        && (next_position.0 as usize) < rows
                        && (next_position.1 as usize) < columns
                    {
                        let next_row = next_position.0 as usize;
                        let next_column = next_position.1 as usize;

                        if lab_map_with_obstruction[next_row][next_column] == "#"
                            || lab_map_with_obstruction[next_row][next_column] == "O"
                        {
                            // If obstructed, turn 90 degrees clockwise
                            let current_index = directions.iter().position(|&d| d == current_direction).unwrap();
                            current_direction = directions[(current_index + 1) % directions.len()];
                        } else {
                            // Otherwise, move forward
                            current_position = (next_row, next_column);
                        }
                    } else {
                        break; // Stop if moving out of bounds
                    }

                }
            }
        }
    }

    positions_for_obstructions
}



fn part2(_input: &str) -> i32 {
    let lab_map = parse_input(_input);
    // println!("{:?}", lab_map);

    let initial_guard_position = find_guard_position(&lab_map).expect("Guard not found in the lab map");
    println!("Guard found at position: {:?}", initial_guard_position);

    let obstruction_positions = simulate_patrol_protocol_with_obstructions(&lab_map, initial_guard_position, (-1, 0));
    // println!("Obstruction positions: {:?}", obstruction_positions);

    obstruction_positions.len() as i32
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
        
    #[test]
    fn test_part2_example() {
        let result = part2("....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...");
        let expected = 6;
        assert_eq!(result, expected);
    }
}
