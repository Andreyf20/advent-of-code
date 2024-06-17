// Read the problem from adventofcode.com
// LEVEL 1 ADVENT OF CODE 2022
// REF: https://adventofcode.com/2022/day/1
use std::fs;

#[derive(Clone)] // we add the Clone trait to Elf struct
struct Elf {
    calories_carried: usize,
}

fn get_sum_top_three_calories(elves_vec: &mut Vec<Elf>) -> usize {
    elves_vec.sort_by_key(|elf| elf.calories_carried);
    elves_vec.reverse();
    elves_vec[0].calories_carried + elves_vec[1].calories_carried + elves_vec[2].calories_carried
}

fn get_max_calories(elves_vec: Vec<Elf>) -> usize {
    let max = elves_vec.iter().max_by_key(|elf| elf.calories_carried);
    match max {
        Some(_) => max.unwrap().calories_carried,
        None => 0,
    }
}

fn get_elf_from_calories(calories_string: Vec<String>) -> Elf {
    let mut elf = Elf {
        calories_carried: 0,
    };
    let mut input_count: usize = 0;
    for number in calories_string {
        let parsed_number = number.parse::<usize>().unwrap();
        input_count += parsed_number;
    }
    elf.calories_carried = input_count;
    elf
}

fn parse_string(input_string: String) -> Vec<Elf> {
    let mut calories_string: Vec<String> = Vec::new();
    let mut elves_vec: Vec<Elf> = Vec::new();
    for i in input_string.lines() {
        let parsed_line = i.trim();
        if !parsed_line.is_empty() {
            calories_string.push(String::from(parsed_line));
        } else if parsed_line.is_empty() && !calories_string.is_empty() {
            elves_vec.push(get_elf_from_calories(calories_string));
            calories_string = Vec::new();
        }
    }
    elves_vec
}

fn main() {
    let file_path = "input.txt";

    let file_content =
        fs::read_to_string(file_path).expect("Missing or cannot open file input.txt");

    let elves_vec = parse_string(file_content);
    let mut elves_vec2 = elves_vec.clone();
    println!("Max calories: {}", get_max_calories(elves_vec));
    println!(
        "Sum of top three: {}",
        get_sum_top_three_calories(&mut elves_vec2)
    );
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*; // <-- import everything from the parent module

    #[test]
    fn part_1() {
        let test_string: String = String::from(
            "
        1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000
        ",
        );

        let elves_vec = parse_string(test_string);
        assert_eq!(get_max_calories(elves_vec), 24000);
    }

    #[test]
    fn part_2() {
        let test_string: String = String::from(
            "
        1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000
        ",
        );

        let mut elves_vec = parse_string(test_string);
        assert_eq!(get_sum_top_three_calories(&mut elves_vec), 45000);
    }
}
