fn main(){
    println!("Hello, day 1 - part 1!");
    
    // Read input file
    let input = include_str!("../../input.txt");
    // dbg!(input);

    // Complete part 1
    let output_part_1 = part1(input);
    dbg!(output_part_1);
    // 2164381

    // Complete part 2
    let output_part_2 = part2(input);
    dbg!(output_part_2);

}

fn is_valid_sequence(numbers: &[i32]) -> bool {
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

        dbg!(&parts);

        are_lines_safe.push(is_valid_sequence(&parts) as i32);
    }
    
    let total_safe: i32 = are_lines_safe.iter().sum();
    total_safe.to_string()
        

}

fn part2(_input: &str) -> String {
    todo!()
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

}
