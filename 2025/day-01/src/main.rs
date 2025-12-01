use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    0
}

fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example_part1() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(part2(&input), 0);
    }
}
