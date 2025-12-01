use std::fs;

fn main() {
    let file_input = fs::read_to_string("input.txt").expect("failed to read input.txt");
    let instructions = parse_input(&file_input);
    
    println!("INSTRUCTIONS 📑 {:#?}", instructions);

    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));
}

#[derive(Debug)]
struct Instruction {
    direction: char,
    times: i32,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let direction = line.chars().next().unwrap();
            let times = line[1..].parse::<i32>().unwrap();
            Instruction { direction, times }
        })
        .collect()
}

fn part1(instructions: &Vec<Instruction>) -> i32 {
    let mut dial_position = 50;
    let mut counter: i32 = 0;

    for instruction in instructions {
        match instruction.direction {
            'L' => dial_position -= instruction.times,
            'R' => dial_position += instruction.times,
            _ => panic!("Unknown direction {}", instruction.direction),
        }

        // Handle wrap-around
        while dial_position < 0 {
            dial_position += 100;
        }

        while dial_position >= 100 {
            dial_position -= 100;
        }

        if dial_position == 0 {
            counter += 1;
        }

        // println!("Current 📍: {}", &dial_position);
    }

    counter
}

fn part2(_input: &Vec<Instruction>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example_part1() {
        let file_input = fs::read_to_string("example.txt").unwrap();
        let input = parse_input(&file_input);
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_example_part2() {
        let file_input = fs::read_to_string("example.txt").unwrap();
        let input = parse_input(&file_input);
        assert_eq!(part2(&input), 0);
    }
}
