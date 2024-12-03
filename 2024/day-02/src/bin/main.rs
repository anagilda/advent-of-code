fn main(){
    println!("Hello, day 2!");
    
    // Read input file
    let input = include_str!("../../input.txt");
    // dbg!(input);

    // Complete part 1
    let output_part_1 = part1(input);
    dbg!(output_part_1);

    // Complete part 2
    let output_part_2 = part2(input);
    dbg!(output_part_2);

}

fn is_sequence_valid(numbers: &[i32]) -> bool {
    if numbers.is_empty() {
        return true;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for window in numbers.windows(2) {
        let diff = window[1] - window[0];

        // Check if the difference violates the constraints
        if diff.abs() > 3 || diff == 0 {
            return false;
        }

        // Update the increasing or decreasing status
        if diff > 0 {
            decreasing = false;
        } else {
            increasing = false;
        }
    }

    increasing || decreasing
}

fn are_both_positive_or_both_negative(a: i32, b: Option<i32>) -> bool {
    match b {
        Some(b_value) => (a > 0 && b_value > 0) || (a < 0 && b_value < 0),
        None => true, // Return true if no previous value to compare so we can continue
    }
}

fn is_sequence_valid_with_problem_dampener(numbers: &[i32], tolerate: bool) -> bool {
    if numbers.is_empty() {
        return true;
    }

    let mut increasing = true;
    let mut decreasing = true;

    let mut previous_diff: Option<i32> = None;

    for (index, window) in numbers.windows(2).enumerate() {
        let diff = window[1] - window[0];
        // dbg!(&tolerate);
        // dbg!(&diff);
        
        // Check if the difference violates the constraints
        if (diff.abs() > 3 || diff == 0) || !are_both_positive_or_both_negative(diff, previous_diff) {
            if !tolerate {
                return false;
            }

            // remove one and the other, and if one of them is valid, return true
            // Create two separate vectors for the two removal cases
            let mut new_list_removed_current_index = numbers.to_vec();
            let mut new_list_removed_index_after = numbers.to_vec();

            // Remove the first and second elements in separate vectors
            new_list_removed_current_index.remove(index);
            new_list_removed_index_after.remove(index + 1);

            // dbg!(&new_list_removed_current_index);
            // dbg!(&new_list_removed_index_after);

            let is_first_sequence_valid = is_sequence_valid_with_problem_dampener(&new_list_removed_current_index, false);
            let is_second_sequence_valid = is_sequence_valid_with_problem_dampener(&new_list_removed_index_after, false);

            // dbg!(is_first_sequence_valid);
            // dbg!(is_second_sequence_valid);

            let mut is_third_sequence_valid = false;
            if index > 0 {
                let mut new_list_removed_index_before = numbers.to_vec();
                new_list_removed_index_before.remove(index-1);
                if is_sequence_valid_with_problem_dampener(&new_list_removed_index_before, false) {
                    is_third_sequence_valid = true;
                }
            }

            return is_first_sequence_valid || is_second_sequence_valid || is_third_sequence_valid;
        }

        // Update the increasing or decreasing status
        if diff > 0 {
            decreasing = false;
        } else {
            increasing = false;
        }

        previous_diff = Some(diff);

    }

    increasing || decreasing
}


fn part1(_input: &str) -> String {

    let mut are_lines_safe: Vec<i32> = Vec::new();
    // Iterate over lines
    for line in _input.lines() {
        // println!("{}", line);

        // Split the line into words by whitespace and collect into a Vec
        let parts: Vec<i32> = line
            .split_whitespace() // Split by whitespace
            .map(|s| s.parse::<i32>().unwrap()) // Parse each part to an i32
            .collect();

        // dbg!(&parts);

        are_lines_safe.push(is_sequence_valid(&parts) as i32);
    }
    
    let total_safe: i32 = are_lines_safe.iter().sum();
    total_safe.to_string()
        

}

fn part2(_input: &str) -> String {
    let mut are_lines_safe: Vec<i32> = Vec::new();
    // Iterate over lines
    for line in _input.lines() {
        // println!("{}", line);

        // Split the line into words by whitespace and collect into a Vec
        let parts: Vec<i32> = line
            .split_whitespace() // Split by whitespace
            .map(|s| s.parse::<i32>().unwrap()) // Parse each part to an i32
            .collect();

        // dbg!(&parts);

        are_lines_safe.push(is_sequence_valid_with_problem_dampener(&parts, true) as i32);
    }

    // dbg!(&are_lines_safe);
    let total_safe: i32 = are_lines_safe.iter().sum();
    total_safe.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_one_line() {
        let result = part1("7 6 4 2 1");
        let expected = 1;
        assert_eq!(result, expected.to_string());
    }
    
    #[test]
    fn test_part1_example() {
        let result = part1("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9");
        let expected = 2;
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part2_one_line() {
        let result = part2("1 3 2 4 5");
        let expected = 1;
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part2_one_line_bug() {
        let result = part2("1 4 3 2 1");
        let expected = 1;
        assert_eq!(result, expected.to_string());
    }
        
    #[test]
    fn test_part2_example() {
        let result = part2("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9");
        let expected = 4;
        assert_eq!(result, expected.to_string());
    }
}
