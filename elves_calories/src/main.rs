// Read the problem from adventofcode.com
// LEVEL 1 ADVENT OF CODE 2022
// REF: https://adventofcode.com/2022/day/1
use std::fs;

struct Elf {
    item_calories: Vec<usize>,
}

impl Elf {
    fn get_calories_count(self) -> usize {
        let mut count = 0;
        for i in self.item_calories {
            count += i;
        }
        count
    }
}

fn get_max_calories(elves_vec: Vec<Elf>) -> usize {
    let mut max_calories = 0;
    for i in elves_vec {
        let calories = i.get_calories_count();
        if calories > max_calories {
            max_calories = calories;
        }
    }
    max_calories
}

fn get_elf_from_calories(calories_string: Vec<String>) -> Elf {
    let mut elf = Elf {
        item_calories: Vec::new(),
    };
    for i in calories_string {
        elf.item_calories.push(i.parse::<usize>().unwrap());
    }
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
    println!("Max calories: {}", get_max_calories(elves_vec));
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*; // <-- import everything from the parent module

    #[test]
    fn it_works() {
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
}
