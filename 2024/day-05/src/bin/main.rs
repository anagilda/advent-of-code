use std::collections::{HashMap, HashSet};

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

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut order_rules = Vec::new();
    let mut manuals_to_print = Vec::new();

    for line in input.lines() {
        if line.contains('|') {
            // Parse pipe-separated pairs
            let parts: Vec<i32> = line
                .split('|')
                .filter_map(|x| x.trim().parse::<i32>().ok())
                .collect();
            if parts.len() == 2 {
                order_rules.push((parts[0], parts[1]));
            }
        } else if line.contains(',') {
            // Parse comma-separated lists
            let parts: Vec<i32> = line
                .split(',')
                .filter_map(|x| x.trim().parse::<i32>().ok())
                .collect();
            if !parts.is_empty() {
                manuals_to_print.push(parts);
            }
        }
    }

    (order_rules, manuals_to_print)
}

fn are_manual_pages_in_order(
    pages: &[i32],
    order_rules: &[(i32, i32)],
) -> bool {
    for &(a, b) in order_rules {
        let mut found_b = false;
        for &page in pages {
            if page == b {
                found_b = true; // Mark that 'b' has been found
            }
            if page == a {
                if found_b {
                    return false; // 'b' found before 'a'; rule violated
                } else {
                    break; // 'b' comes after 'a', or not at all; rule satisfied
                }
            }
        }
    }
    true // All rules satisfied
}

fn find_middle_pages(valid_manuals: &[Vec<i32>]) -> Vec<i32> {
    valid_manuals
        .iter()
        .filter_map(|list| {
            if list.is_empty() {
                None // Skip empty lists
            } else {
                Some(list[list.len() / 2]) // Find middle item
            }
        })
        .collect()
}

fn part1(_input: &str) -> i32 {
    let (order_rules, manuals_to_print) = parse_input(_input);

    println!("Order Rules: {:?}", order_rules);
    println!("Manual Pages to Print: {:?}", manuals_to_print);

    let correct_manuals_to_print: Vec<Vec<i32>> = manuals_to_print
        .iter()
        .filter(|pages| are_manual_pages_in_order(pages, &order_rules))
        .cloned()
        .collect();

    // println!("{}",are_manual_pages_in_order(&manuals_to_print[0], &order_rules));
    // println!("Correct Manual Pages to Print: {:?}", correct_manuals_to_print);

    let middle_items = find_middle_pages(&correct_manuals_to_print);
    
    
    middle_items.iter().sum()
}

fn order_manual(pages: &[i32], order_rules: &[(i32, i32)]) -> Vec<i32> {
    // Create a mapping of all pages that succeed each page
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    for &(a, b) in order_rules {
        graph.entry(a).or_default().insert(b);
    }

    // Sort pages based on graph precedence
    let mut ordered_pages = pages.to_vec();
    ordered_pages.sort_by(|&page_a, &page_b| {
        if is_preceding(page_a, page_b, &graph) {
            // 'a' should come before 'b'
            std::cmp::Ordering::Less
        } else if is_preceding(page_b, page_a, &graph) {
            // 'a' should come after 'b'
            std::cmp::Ordering::Greater
        } else {
            // No need to change order
            std::cmp::Ordering::Equal
        }
    });

    ordered_pages
}

fn is_preceding(a: i32, b: i32, graph: &HashMap<i32, HashSet<i32>>) -> bool {
    // Check if 'a' should precede 'b' in the graph
    if let Some(precedes) = graph.get(&a) {
        precedes.contains(&b)
    } else {
        false
    }
}


fn part2(_input: &str) -> i32 {
    let (order_rules, manuals_to_print) = parse_input(_input);

    let invalid_manuals: Vec<Vec<i32>> = manuals_to_print
        .iter()
        .filter(|pages| !are_manual_pages_in_order(pages, &order_rules))
        .cloned()
        .collect();

    let ordered_invalid_manuals: Vec<Vec<i32>> = invalid_manuals
        .iter()
        .map(|manual| order_manual(manual, &order_rules))
        .collect();

    let middle_items = find_middle_pages(&ordered_invalid_manuals);
    
    
    middle_items.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_one_line() {
        let result = part1("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29");
        let expected = 61;
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_part1_example() {
        let result = part1("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47");
        let expected = 143;
        assert_eq!(result, expected);
    }
        
    #[test]
    fn test_part2_example() {
        let result = part2("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47");
        let expected = 123;
        assert_eq!(result, expected);
    }
}
