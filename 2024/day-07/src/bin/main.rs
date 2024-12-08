use itertools::Itertools;
use std::collections::HashMap;

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


fn evaluate_expression(expression: &str) -> i64 {
    let mut tokens = vec![];
    let mut number = String::new();

    // Iterate through each character in the expression
    for c in expression.chars() {
        if c.is_digit(10) {
            number.push(c); // Collect digits of the number
        } else {
            if !number.is_empty() {
                tokens.push(number.clone()); // Add the number to tokens
                number.clear(); // Reset number for next digits
            }
            tokens.push(c.to_string()); // Add the operator to tokens
        }
    }
    
    // Push the last number if there's one remaining
    if !number.is_empty() {
        tokens.push(number);
    }

    // Now evaluate the expression based on the tokens
    let mut result = tokens
        .get(0)
        .and_then(|num| num.parse::<i64>().ok())
        .expect("Expression must start with a number");

    // Iterate through the tokens
    let mut i = 1;
    while i < tokens.len() {
        let operator = &tokens[i];
        let next_num = tokens.get(i + 1)
            .and_then(|num| num.parse::<i64>().ok())
            .expect("Invalid number after operator");

        match operator.as_str() {
            "+" => result += next_num,  // Add to the result
            "*" => result *= next_num,  // Multiply to the result
            _ => panic!("Unexpected operator: {}", operator),
        }

        i += 2; // Skip over the operator and the next number
    }

    result
}

fn can_equal_total_in_order(numbers: &[i64], total: i64) -> bool {
    let n = numbers.len();
    let operators = vec!['+', '*'];

    // Generate all combinations of operators between the numbers
    let operator_combinations = (0..n - 1).map(|_| &operators).multi_cartesian_product();

    // dbg!(&operator_combinations);

    for op_comb in operator_combinations {
        let mut expression = String::new();
        for (i, &num) in numbers.iter().enumerate() {
            if i > 0 {
                expression.push(*op_comb[i - 1]);
            }
            expression.push_str(&num.to_string());
        }

        dbg!(&expression);

        // Evaluate the expression
        if evaluate_expression(&expression) == total {
            return true;
        }
    }

    false
}


fn check_expression_is_valid(line: &str) -> bool {
    // Split the line into total and numbers
    if let Some((total_str, numbers_str)) = line.split_once(':') {
        let total: i64 = total_str.trim().parse().expect("Invalid total number");
        let numbers: Vec<i64> = numbers_str
            .trim()
            .split_whitespace()
            .map(|num| num.parse().expect("Invalid number in list"))
            .collect();

        return can_equal_total_in_order(&numbers, total);
    }
    false
}

fn process_input(input: &str) -> HashMap<i64, bool> {
    let mut results = HashMap::new();
    for line in input.lines() {
        if !line.trim().is_empty() {
            let valid = check_expression_is_valid(line);

            // dbg!(&valid);
            if let Some((total_str, _)) = line.split_once(':') {
                let total: i64 = total_str.trim().parse().expect("Invalid total number");
                results.insert(total, valid);
            }
        }
    }
    results
}

fn parse_input(input: &str) -> HashMap<i64, Vec<i64>> {
    let mut result = HashMap::new();

    for line in input.lines() {
        // Split each line into the total and the list of numbers
        if let Some((total_str, numbers_str)) = line.split_once(':') {
            let total: i64 = total_str.trim().parse().expect("Invalid total number");
            let numbers: Vec<i64> = numbers_str
                .trim()
                .split_whitespace()
                .map(|num| num.parse().expect("Invalid number in list"))
                .collect();
            result.insert(total, numbers);
        }
    }

    result
}


fn part1(input: &str) -> i64 {
    let equation_numbers = parse_input(input);
    println!("{:?}", equation_numbers);

    let results = process_input(input);

    // dbg!(&results);

    results.iter()
    .filter(|&(_, &is_true)| is_true)  // Filter only the ones where value is true
    .map(|(&key, _)| key)               // Map the keys
    .sum()                              // Sum the keys
}



fn part2(_input: &str) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part1_example() {
        let result = part1("190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20");
        let expected = 3749;
        assert_eq!(result, expected);
    }
}
