use std::collections::HashMap;
use std::collections::HashSet;

type Coordinate = (i32, i32);


fn main(){
    println!("Hello, day 3!");
    
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

fn parse_input(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_string()).collect())
        .collect()
}

fn map_antennas_to_antinodes(grid: &[Vec<String>]) -> HashMap<String, Vec<Coordinate>> {
    let mut antennas_coordinates: HashMap<String, Vec<Coordinate>> = HashMap::new();

    for (row, line) in grid.iter().enumerate() {
        for (column, character) in line.iter().enumerate() {
            if *character != '.'.to_string() {
                antennas_coordinates
                    .entry(character.to_string())
                    .or_insert_with(Vec::new)
                    .push((row as i32, column as i32));
            }
        }
    }

    calculate_antinodes(&antennas_coordinates, (grid.len() as i32, grid[0].len() as i32))
}

fn calculate_antinodes(
    antennas_coordinates: &HashMap<String, Vec<Coordinate>>,
    grid_size: (i32, i32), // (rows, columns)
) -> HashMap<String, Vec<Coordinate>> {
    let mut antinodes: HashMap<String, Vec<Coordinate>> = HashMap::new();

    for (&ref character, coordinates) in antennas_coordinates.iter() {
        let mut character_antinodes = Vec::new();

        for i in 0..coordinates.len() {
            for j in i + 1..coordinates.len() {
                let (x1, y1) = coordinates[i];
                let (x2, y2) = coordinates[j];

                // Calculate the distance (dx, dy)
                let dx = x2 - x1;
                let dy = y2 - y1;

                // Calculate new coordinates for the antinodes
                let antinode1 = (x1 - dx, y1 - dy);
                let antinode2 = (x2 + dx, y2 + dy);

                dbg!(&antinode1, &antinode2);

                // Check if the antinodes are within bounds
                if is_within_bounds(antinode1, grid_size) {
                    character_antinodes.push(antinode1);
                    dbg!(&character, &coordinates, &i, &j, &x1, &y1, &x2, &y2);

                }
                if is_within_bounds(antinode2, grid_size) {
                    character_antinodes.push(antinode2);
                    dbg!(&character, &coordinates, &i, &j, &x1, &y1, &x2, &y2);

                }
            }
        }

        antinodes.insert(character.to_string(), character_antinodes);
    }

    antinodes
}

fn is_within_bounds(coord: Coordinate, grid_size: (i32, i32)) -> bool {
    let (rows, cols) = grid_size;
    let (x, y) = coord;
    x >= 0 && y >= 0 && x < rows && y < cols
}

fn part1(input: &str) -> i32 {
    let antenna_map = parse_input(input);
    // dbg!(&antenna_map);

    let antennas_to_antinodes = map_antennas_to_antinodes(&antenna_map);
    dbg!(&antennas_to_antinodes);

    let unique_coordinates: HashSet<(i32, i32)> = antennas_to_antinodes
    .values()
    .flat_map(|coords| coords.iter().cloned())
    .collect();

    unique_coordinates.len() as i32
}



fn part2(_input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part1_example() {
        let result = part1("............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............");
        let expected = 14;
        assert_eq!(result, expected);
    }
}
