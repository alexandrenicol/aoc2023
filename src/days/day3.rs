use std::error::Error;
use crate::helpers::file_helper::read_lines;

use regex::{Match, Regex};

#[derive(Debug)]
#[derive(Clone)]
struct Number {
    number: i32,
    start: i32,
    end: i32,
    row: i32,
}

#[derive(Debug)]
#[derive(Clone)]
struct SpecialChar {
    char: String,
    position: i32,
    row: i32,
}

fn match_to_number (a_match: Match, row_number: i32) -> Number {
    Number {
        number: a_match.as_str().parse::<i32>().unwrap(),
        start: a_match.start() as i32,
        end: (a_match.end()-1) as i32,
        row:  row_number
    }
}

fn match_to_special_char (a_match: Match, row_number: i32) -> SpecialChar {
    SpecialChar {
        char: a_match.as_str().to_string(),
        position: a_match.start() as i32,
        row:  row_number
    }
}

fn filter_special_chars_by_adjacent_row (special_chars: Vec<SpecialChar>, row_number: i32) -> Vec<SpecialChar> {
    special_chars.into_iter().filter(|special_char| special_char.row == row_number || special_char.row == row_number - 1 || special_char.row == row_number + 1).collect()
}

fn filter_numbers_by_adjacent_row (numbers: Vec<Number>, row_number: i32) -> Vec<Number> {
    numbers.into_iter().filter(|number| number.row == row_number || number.row == row_number - 1 || number.row == row_number + 1).collect()
}

pub fn day3_1 () -> Result<i32, Box<dyn Error>> {
    let number_regex = Regex::new(r"[0-9]+").unwrap();

    let special_char_regex = Regex::new(r"[^\d.]").unwrap();


    let lines = read_lines("./data/day3/input.txt");

    let mut numbers: Vec<Number> = Vec::new();
    let mut special_chars: Vec<SpecialChar> = Vec::new();

    let mut part_numbers: Vec<Number> = Vec::new();

    for i in 0..lines.len() {
        let line = &lines[i];

        let line_numbers= find_numbers_in_line(line, i, number_regex.clone());

        let line_special_chars = find_special_chars_in_line(line, i, special_char_regex.clone());

        numbers.extend(line_numbers);
        special_chars.extend(line_special_chars);
    }

    for number in numbers {

        let adjacent_special_chars = filter_special_chars_by_adjacent_row(
            special_chars.clone(),
            number.row
        );

        for special_char in adjacent_special_chars {

            let valid_x_position= special_char.position <= number.end + 1 && special_char.position >= number.start - 1;

            if valid_x_position {
                part_numbers.push(number.clone());
            }
        }
    }

    let sum = part_numbers.iter().map(|number| number.number).sum();

    Ok(sum)
}

pub fn day3_2 () -> Result<i32, Box<dyn Error>> {
    let number_regex = Regex::new(r"[0-9]+").unwrap();

    let special_char_regex = Regex::new(r"[*]").unwrap();

    let lines = read_lines("./data/day3/input.txt");

    let mut numbers: Vec<Number> = Vec::new();
    let mut special_chars: Vec<SpecialChar> = Vec::new();

    let mut gear_ratios: Vec<i32> = Vec::new();

    for i in 0..lines.len() {
        let line = &lines[i];

        let line_numbers= find_numbers_in_line(line, i, number_regex.clone());

        let line_special_chars = find_special_chars_in_line(line, i, special_char_regex.clone());

        numbers.extend(line_numbers);
        special_chars.extend(line_special_chars);
    }

    for special_char in special_chars {

        let adjacent_numbers = filter_numbers_by_adjacent_row(
            numbers.clone(),
            special_char.row
        );

        // filter numbers by x position
        let adjacent_numbers: Vec<Number> = adjacent_numbers.into_iter().filter(|number| special_char.position <= number.end + 1 && special_char.position >= number.start - 1).collect();

        if adjacent_numbers.len() == 2 {
            gear_ratios.push(adjacent_numbers[0].number * adjacent_numbers[1].number);
        }
    }

    let sum = gear_ratios.iter().sum();

    Ok(sum)
}


fn find_numbers_in_line(line: &str, line_index: usize, number_regex: Regex) -> Vec<Number> {
    let numbers  = number_regex.find_iter(&*line).map(|m| match_to_number(m, line_index as i32)).collect();
    numbers
}

fn find_special_chars_in_line(line: &str, line_index: usize, special_char_regex: Regex) -> Vec<SpecialChar> {
    let special_chars  = special_char_regex.find_iter(&*line).map(|m| match_to_special_char(m, line_index as i32)).collect();
    special_chars
}
