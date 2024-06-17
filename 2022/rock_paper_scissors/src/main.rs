// Read the problem from adventofcode.com
// LEVEL 2 ADVENT OF CODE 2022
// REF: https://adventofcode.com/2022/day/2
use std::fs;

enum PlayInput {
    ROCK,
    PAPER,
    SCISSORS,
}

fn get_input(input: &char) -> PlayInput {
    match input {
        'A' | 'X' => PlayInput::ROCK,
        'B' | 'Y' => PlayInput::PAPER,
        'C' | 'Z' => PlayInput::SCISSORS,
        _ => panic!("Invalid input"),
    }
}

fn get_input_part_2(elf_input: &PlayInput, result_input: &char) -> PlayInput {
    // X - LOSE, Y - DRAW, Z - WIN
    match elf_input {
        PlayInput::ROCK => match result_input {
            'X' => PlayInput::SCISSORS,
            'Y' => PlayInput::ROCK,
            'Z' => PlayInput::PAPER,
            _ => panic!("Invalid input"),
        },
        PlayInput::PAPER => match result_input {
            'X' => PlayInput::ROCK,
            'Y' => PlayInput::PAPER,
            'Z' => PlayInput::SCISSORS,
            _ => panic!("Invalid input"),
        },
        PlayInput::SCISSORS => match result_input {
            'X' => PlayInput::PAPER,
            'Y' => PlayInput::SCISSORS,
            'Z' => PlayInput::ROCK,
            _ => panic!("Invalid input"),
        },
    }
}

fn play_round(elf_input: PlayInput, player_input: PlayInput) -> usize {
    // Constants =  1 - ROCK, 2 - PAPER, 3 - SCISSORS
    // Result    =  0 - LOSS, 3 - DRAW,  6 - WIN
    // Constant + Result
    match elf_input {
        PlayInput::ROCK => match player_input {
            PlayInput::ROCK => 1 + 3,
            PlayInput::PAPER => 2 + 6,
            PlayInput::SCISSORS => 3 + 0,
        },
        PlayInput::PAPER => match player_input {
            PlayInput::ROCK => 1 + 0,
            PlayInput::PAPER => 2 + 3,
            PlayInput::SCISSORS => 3 + 6,
        },
        PlayInput::SCISSORS => match player_input {
            PlayInput::ROCK => 1 + 6,
            PlayInput::PAPER => 2 + 0,
            PlayInput::SCISSORS => 3 + 3,
        },
    }
}

fn parse_string_and_play(input_string: String, is_part_2: bool) -> usize {
    let mut total_score = 0;
    for line in input_string.lines() {
        let parsed_line = line.trim();
        if !parsed_line.is_empty() {
            let elf_input = get_input(&parsed_line.chars().next().unwrap());
            let player_input = if is_part_2 {
                get_input_part_2(&elf_input, &parsed_line.chars().last().unwrap())
            } else {
                get_input(&parsed_line.chars().last().unwrap())
            };
            total_score += play_round(elf_input, player_input);
        }
    }
    total_score
}

fn main() {
    let file_path = "input.txt";

    let file_content =
        fs::read_to_string(file_path).expect("Missing or cannot open file input.txt");

    let result = parse_string_and_play(file_content, true);
    println!("Result = {:?}", result);
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*; // <-- import everything from the parent module

    #[test]
    fn part_1() {
        let test_input = String::from(
            "
        A Y
        B X
        C Z
        ",
        );

        let result = parse_string_and_play(test_input, false);

        assert_eq!(result, 15);
    }

    #[test]
    fn part_2() {
        let test_input = String::from(
            "
        A Y
        B X
        C Z
        ",
        );

        let result = parse_string_and_play(test_input, true);

        assert_eq!(result, 12);
    }
}
