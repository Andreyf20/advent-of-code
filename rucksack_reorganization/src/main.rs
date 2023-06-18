// Read the problem from adventofcode.com
// LEVEL 3 ADVENT OF CODE 2022
// REF: https://adventofcode.com/2022/day/3
use std::fs;

fn get_real_value_from_ascii(ch: u8) -> i32 {
    if ch > 97 {
        return (ch - 96) as i32;
    } else {
        return (ch - 38) as i32;
    }
}

fn match_items(first_half: &str, second_half: &str) -> i32 {
    for i in first_half.chars() {
        for j in second_half.chars() {
            if i == j {
                return get_real_value_from_ascii(i as u8);
            }
        }
    }
    panic!("No match found");
}

fn parse_string(input_string: String) -> i32 {
    let mut count = 0;
    for line in input_string.lines() {
        let parsed_line = line.trim();
        if !parsed_line.is_empty() {
            let first_half = &parsed_line[0..(parsed_line.len() / 2)];
            let second_half = &parsed_line[(parsed_line.len() / 2)..parsed_line.len()];
            let real_value = match_items(first_half, second_half) as i32;
            count += real_value;
        }
    }
    count
}

fn main() {
    let file_path = "input.txt";

    let file_content =
        fs::read_to_string(file_path).expect("Missing or cannot open file input.txt");

    let result = parse_string(file_content);

    println!("The sum is: {}", result);
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*; // <-- import everything from the parent module

    #[test]
    fn part_1() {
        let test_string: String = String::from(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );

        let result = parse_string(test_string);
        assert_eq!(result, 157);
    }
}
