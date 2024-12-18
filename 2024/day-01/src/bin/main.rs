fn main(){
    println!("Hello, day 1!");
    
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

fn part1(_input: &str) -> String {
    let mut first_group_locations: Vec<i32> = Vec::new();
    let mut second_group_locations: Vec<i32> = Vec::new();

    // Iterate over lines
    for line in _input.lines() {
        // println!("{}", line);

        // Split the line into words by whitespace and collect into a Vec
        let parts: Vec<i32> = line
            .split_whitespace() // Split by whitespace
            .map(|s| s.parse::<i32>().unwrap()) // Parse each part to an i32
            .collect();

        // Ensure there are exactly two integers
        if parts.len() != 2 {
            println!("The line does not contain exactly two integers.");
        }

        let (first_location_id, second_location_id) = (parts[0], parts[1]);
        first_group_locations.push(first_location_id);
        second_group_locations.push(second_location_id);
    }

    first_group_locations.sort();
    second_group_locations.sort();

    let mut distances: Vec<i32> = Vec::new();

    for (first_location_id, second_location_id) in first_group_locations.iter().zip(second_group_locations.iter()) {
        // println!("a: {}, b: {}", first_location_id, second_location_id);
        let distance = (first_location_id - second_location_id).abs();
        distances.push(distance);
    }
    
    // dbg!(&distances);

    let total_distance: i32 = distances.iter().sum();
    total_distance.to_string()
}

fn part2(_input: &str) -> String {
    // Repeated code to refactor
    let mut first_group_locations: Vec<i32> = Vec::new();
    let mut second_group_locations: Vec<i32> = Vec::new();

    // Iterate over lines
    for line in _input.lines() {
        // println!("{}", line);

        // Split the line into words by whitespace and collect into a Vec
        let parts: Vec<i32> = line
            .split_whitespace() // Split by whitespace
            .map(|s| s.parse::<i32>().unwrap()) // Parse each part to an i32
            .collect();

        // Ensure there are exactly two integers
        if parts.len() != 2 {
            println!("The line does not contain exactly two integers.");
        }

        let (first_location_id, second_location_id) = (parts[0], parts[1]);
        first_group_locations.push(first_location_id);
        second_group_locations.push(second_location_id);
    }
    // End of repeated code to refactor

    // Create a Vec to store the counts
    let mut counts: Vec<i32> = Vec::new();

    // Iterate over list1 and count occurrences in list2
    for &location_id in &first_group_locations {
        let count = second_group_locations.iter().filter(|&&x| x == location_id).count() as i32;
        let points = location_id * count;
        counts.push(points);
    }
    // dbg!(&counts);
    
    let total_count: i32 = counts.iter().sum();
    total_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_one_line() {
        let result = part1("3 2");
        let expected = 1;
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part1_example() {
        let result = part1("3   4
4   3
2   5
1   3
3   9
3   3");
        let expected = 11;
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part2_example() {
        let result = part2("3   4
4   3
2   5
1   3
3   9
3   3");
        let expected = 31;
        assert_eq!(result, expected.to_string());
    }
}
