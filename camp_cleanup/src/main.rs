// Read the problem from adventofcode.com
// LEVEL 1 ADVENT OF CODE 2022
// REF: https://adventofcode.com/2022/day/4
use std::fs;

fn get_numeric_values(elf_input: &str) -> (usize, usize) {
    let mut parser = String::new();
    let mut result = (0, 0);
    for ch in elf_input.chars() {
        if ch.is_numeric() {
            parser.push(ch);
        } else {
            let value = parser.parse::<usize>().unwrap();
            result.0 = value;
            parser = String::new();
        }
    }
    if !parser.is_empty() {
        result.1 = parser.parse::<usize>().unwrap();
    }
    result
}

fn check_is_contained(first_elf: &str, second_elf: &str) -> bool {
    let (first_elf_first_number, first_elf_last_number) = get_numeric_values(first_elf);
    let (second_elf_first_number, second_elf_last_number) = get_numeric_values(second_elf);

    if first_elf_first_number <= second_elf_first_number
        && first_elf_last_number >= second_elf_last_number
        || first_elf_first_number >= second_elf_first_number
            && first_elf_last_number <= second_elf_last_number
    {
        return true;
    }
    false
}

fn check_is_contained_2(first_elf: &str, second_elf: &str) -> bool {
    let (first_elf_first_number, first_elf_last_number) = get_numeric_values(first_elf);
    let (second_elf_first_number, second_elf_last_number) = get_numeric_values(second_elf);

    // Overlaps only in one point
    if first_elf_first_number == second_elf_last_number
        || first_elf_last_number == second_elf_first_number
    {
        return true;
    }

    // Overlaps inside the range
    if second_elf_first_number >= first_elf_first_number
        && second_elf_first_number <= first_elf_last_number
        || first_elf_first_number >= second_elf_first_number
            && first_elf_first_number <= second_elf_last_number
    {
        return true;
    }

    // Old check to see in range is contained
    if first_elf_first_number <= second_elf_first_number
        && first_elf_last_number >= second_elf_last_number
        || first_elf_first_number >= second_elf_first_number
            && first_elf_last_number <= second_elf_last_number
    {
        return true;
    }
    false
}

fn parse_string(input_string: String, is_part_2: bool) -> usize {
    let mut contained_count: usize = 0;
    for line in input_string.lines() {
        let parsed_line = line.trim();
        if !parsed_line.is_empty() {
            let elfs_inputs: Vec<&str> = parsed_line.split(",").collect();
            let result = if is_part_2 {
                check_is_contained_2(elfs_inputs[0], elfs_inputs[1])
            } else {
                check_is_contained(elfs_inputs[0], elfs_inputs[1])
            };
            if result {
                contained_count += 1;
            }
        }
    }
    contained_count
}

fn main() {
    let file_path = "input.txt";

    let file_content =
        fs::read_to_string(file_path).expect("Missing or cannot open file input.txt");

    let result = parse_string(file_content, false);
    println!("Result: {}", result);

    let file_content =
        fs::read_to_string(file_path).expect("Missing or cannot open file input.txt");

    let result = parse_string(file_content, true);
    println!("Result: {}", result);
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*; // <-- import everything from the parent module

    #[test]
    fn part_1() {
        let test_string: String = String::from(
            "
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        ",
        );

        let result = parse_string(test_string, false);
        assert_eq!(result, 2);
    }

    #[test]
    fn part_2() {
        let test_string: String = String::from(
            "
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        ",
        );

        let result = parse_string(test_string, true);
        assert_eq!(result, 4);
    }
}
