use regex::Regex;

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

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn part1(_input: &str) -> i32 {

    let mut all_tuples: Vec<(i32, i32)> = Vec::new();
    // Iterate over lines
    for line in _input.lines() {
        // println!("{}", line);

        let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";

        // Compile the regex
        let regex = Regex::new(pattern).unwrap();

        // Find all matches
        let tuples: Vec<(i32, i32)> = regex
        .captures_iter(line)
        .map(|caps| {
            let first = caps[1].parse::<i32>().unwrap();
            let second = caps[2].parse::<i32>().unwrap();
            (first, second)
        })
        .collect();

        // Print the result
        // println!("{:?}", tuples); // Output: [(2, 4), (5, 5), (11, 8), (8, 5)]
        all_tuples.extend(tuples)

    }
    
    // dbg!(&all_tuples);

    // Call the function for each tuple and sum the results
    let total: i32 = all_tuples.iter().map(|&(a, b)| mul(a, b)).sum();

    // Print the total sum
    // println!("Total Sum: {}", total);
    total
}

fn part2(_input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_one_line_1() {
        let result = part1("mul(44,46)");
        let expected = 2024;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_one_line_2() {
        let result = part1("mul(123,4)%mul ( 2 , 4 )");
        let expected = 492;
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_part1_example() {
        let result = part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
");
        let expected = 161;
        assert_eq!(result, expected);
    }
}
