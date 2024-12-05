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

fn find_xmas(grid: &[Vec<String>]) -> Vec<(usize, usize, String)> {
    let target = "XMAS";
    let target_rev: String = target.chars().rev().collect();
    
    let mut found_positions = Vec::new();
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

    let rows = grid.len();
    let cols = grid[0].len();

    // Iterate characters
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] != "X" {
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
                if word == target || word == target_rev {
                    found_positions.push((i, j, word));
                }
            }
        }
    }
    found_positions
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
}
