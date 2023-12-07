use crate::helpers::file_helper::read_lines;
use regex::Regex;
use std::error::Error;

pub fn day4_1() -> Result<i32, Box<dyn Error>> {
    let lines = read_lines("./data/day4/input.txt");

    let number_regex = Regex::new(r"[0-9]+").unwrap();

    let mut points: Vec<i32> = Vec::new();

    for line in lines {
        println!("{}", line);

        let split_line: Vec<&str> = line.split(":").collect::<Vec<&str>>()[1]
            .split("|")
            .collect();
        let left = split_line[0].trim();
        let right = split_line[1].trim();

        let winning_numbers: Vec<i32> = number_regex
            .find_iter(&*left)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect();
        let player_numbers: Vec<i32> = number_regex
            .find_iter(&*right)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect();

        let number_of_winning_numbers_in_player_numbers = winning_numbers
            .iter()
            .filter(|winning_number| player_numbers.contains(winning_number))
            .count();

        if number_of_winning_numbers_in_player_numbers > 0 {
            let point = i32::pow(2, (number_of_winning_numbers_in_player_numbers - 1) as u32);
            points.push(point);
        }
    }

    let sum = points.iter().sum::<i32>();

    Ok(sum)
}
