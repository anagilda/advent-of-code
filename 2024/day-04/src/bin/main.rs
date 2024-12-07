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

type Direction = (isize, isize);

fn find_xmas(grid: &[Vec<String>]) -> Vec<(usize, usize, String, Direction)> {
    let directions = [
        (0, 1),  // Horizontal right
        (0, -1), // Horizontal left
        (1, 0),  // Vertical down
        (-1, 0), // Vertical up
        (1, 1),  // Diagonal down-right
        (-1, -1), // Diagonal up-left
        (1, -1), // Diagonal down-left
        (-1, 1), // Diagonal up-right
    ];

    return find_word(grid, "XMAS", directions.to_vec());
}

fn find_word(grid: &[Vec<String>], target: &str, directions: Vec<Direction>) -> Vec<(usize, usize, String, Direction)> {
    
    let mut found_positions = Vec::new();


    let rows = grid.len();
    let cols = grid[0].len();

    // Iterate characters
    for i in 0..rows {
        for j in 0..cols {
            let first_char = target.chars().next().expect("").to_string();
            if grid[i][j] != first_char {
                continue;
            }

            for &(direction_x, direction_y) in &directions {
                let mut word = String::new();
                for target_character_index in 0..target.len() as isize {
                    let x = i as isize + target_character_index * direction_x;
                    let y = j as isize + target_character_index * direction_y;

                    // Check if out of bounds
                    if x < 0 || y < 0 || x >= rows as isize || y >= cols as isize {
                        break;
                    }
                    
                    word.push(grid[x as usize][y as usize].chars().next().unwrap());
                }
                // store where it starts, and the word
                if word == target {
                    found_positions.push((i, j, word, (direction_x, direction_y)));
                }
            }
        }
    }
    found_positions
}

fn find_cross_mas(grid: &[Vec<String>]) -> Vec<Direction> {
    let target = "MAS";
    let target_rev: String = target.chars().rev().collect();
    
    // let candidates_up_right = find_word(grid, target, [(-1, 1)].to_vec());
    // let candidates_down_left = find_word(grid, target, [(1, -1)].to_vec());

    let mut candidates_down_right = find_word(grid, target, [(1, 1)].to_vec());
    candidates_down_right.extend(find_word(grid, &target_rev, [(1, 1)].to_vec()));
    let mut candidates_down_left = find_word(grid, target, [(1, -1)].to_vec());
    candidates_down_left.extend(find_word(grid, &target_rev, [(1, -1)].to_vec()));

    dbg!(&candidates_down_right);
    dbg!(&candidates_down_left);


    // let candidates_first_line = candidates_up_right + &candidates_down_left;
    // let candidates_second_line = candidates_down_left + &candidates_down_right;
    
    // Extract centers from the up-left and down-right candidates
    let centers_down_right: Vec<(isize, isize)> = candidates_down_right
    .iter()
    .map(|&(i, j, _, (direction_x, direction_y))| {
        // Use the original types of direction_x and direction_y, not cast them here
        (i as isize + direction_x, j as isize + direction_y)
    })
    .collect();

    let centers_up_left: Vec<(isize, isize)> = candidates_down_left
    .iter()
    .map(|&(i, j, _, (direction_x, direction_y))| {
        // Same as above, use the original types
        (i as isize + direction_x, j as isize + direction_y)
    })
    .collect();

    dbg!(&centers_down_right);
    dbg!(&centers_up_left);

    // Find matches between the two sets of centers
    let mut centers = Vec::new();

    for center in &centers_up_left {
        if centers_down_right.contains(center) {
            centers.push(*center);
        }
    }
    dbg!(&centers);

    centers

}

fn part1(_input: &str) -> i32 {
    // Parse into a list of lists
    let grid: Vec<Vec<String>> = _input
    .lines() // Split by lines
    .map(|line| {
        line.chars() // Split line into characters
            .map(|ch| ch.to_string()) // Convert each character to String
            .collect() // Collect into a Vec<String>
    })
    .collect(); // Collect all lines into Vec<Vec<String>>

    // Print the resulting list of lists
    // for line in &grid {
    //     println!("{:?}", line);
    // }
    
    let results = find_xmas(&grid);
    dbg!(&results);
    results.len() as i32
}

fn part2(_input: &str) -> i32 {
    // Parse into a list of lists
    let grid: Vec<Vec<String>> = _input
    .lines() // Split by lines
    .map(|line| {
        line.chars() // Split line into characters
            .map(|ch| ch.to_string()) // Convert each character to String
            .collect() // Collect into a Vec<String>
    })
    .collect(); // Collect all lines into Vec<Vec<String>>

    // Print the resulting list of lists
    for line in &grid {
        println!("{:?}", line);
    }
    
    let results = find_cross_mas(&grid);
    dbg!(&results);
    results.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_one_line() {
        let result = part1("MMMSXXMASAMX");
        let expected = 2;
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_part1_example() {
        let result = part1("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX");
        let expected = 18;
        assert_eq!(result, expected);
    }
        
    #[test]
    fn test_part2_example() {
        let result = part2("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX");
        let expected = 9;
        assert_eq!(result, expected);
    }
}
