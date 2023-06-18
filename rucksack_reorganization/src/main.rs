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

fn triple_match_items(first_item: &str, second_item: &str, third_item: &str) -> i32 {
    let result: Vec<char> = first_item
        .chars()
        .filter(|&c| second_item.contains(c) && third_item.contains(c))
        .collect(); // Important this gets me the final value
    get_real_value_from_ascii(result[0] as u8)
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

fn parse_string_part_2(input_string: String) -> i32 {
    let mut group: Vec<&str> = Vec::new();
    let mut count = 0;
    for line in input_string.lines() {
        let parsed_line = line.trim();
        if !parsed_line.is_empty() && group.len() < 3 {
            group.push(parsed_line);
        } else if group.len() == 3 {
            let first_item = group[0];
            let second_item = group[1];
            let third_item = group[2];
            let real_value = triple_match_items(first_item, second_item, third_item);
            count += real_value;
            group = Vec::new();
            group.push(parsed_line);
        }
    }
    let last_real_value = triple_match_items(group[0], group[1], group[2]);
    count + last_real_value
}

fn main() {
    let file_path = "input.txt";

    let file_content =
        fs::read_to_string(file_path).expect("Missing or cannot open file input.txt");

    let result = parse_string(file_content);

    println!("The sum is: {}", result);

    let file_content =
        fs::read_to_string(file_path).expect("Missing or cannot open file input.txt");

    let result_part_2 = parse_string_part_2(file_content);

    println!("The second sum is: {}", result_part_2);
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

    #[test]
    fn part_2() {
        let test_string: String = String::from(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );

        let result = parse_string_part_2(test_string);
        assert_eq!(result, 70);
    }
}
