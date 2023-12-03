use std::{error::Error};
use csv::StringRecord;

// remove all letters from a string and keep only numbers
fn remove_letters(s: &str) -> String {
    s.chars().filter(|c| c.is_digit(10)).collect()
}

// transform a string into an array of numbers
fn string_to_array(s: &str) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    for c in s.chars() {
        v.push(c.to_digit(10).unwrap() as i32);
    }
    v
}

// create a string concatenation of two numbers
fn concat_numbers(a: i32, b: i32) -> String {
    let mut s = a.to_string();
    s.push_str(&b.to_string());
    s
}


fn a_function(line: String) -> (i32, i32) {
    let digits = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    // loop through the array to find the index of the spelled digit
    let mut index = 0;

    // create an empty array of tuples (index, spelled_digit)
    let mut spelled_digits_with_index: Vec<(usize, &str)> = Vec::new();

    for i in 0..digits.len() {
        let digit = digits[i];

        // find the position of the spelled digit in the line
        let position = line.find(digit);
        match position {
            Some(pos) => {
                spelled_digits_with_index.append(&mut vec![(pos, digit)]);
            },
            None => continue,
        }

        // in case the digit appears twice, we also find the last position
        let r_position = line.rfind(digit);
        match r_position {
            Some(pos) => {
                spelled_digits_with_index.append(&mut vec![(pos, digit)]);
            },
            None => continue,
        }
    }

    // sort the array by position
    spelled_digits_with_index.sort_by(|a, b| a.0.cmp(&b.0));

    // get the index of the first digit
    let first_index = spelled_digits_with_index[0].1;

    // get the index of the last digit
    let last_index = spelled_digits_with_index[spelled_digits_with_index.len() - 1].1;

    // find index of first digit from the digits array
    let mut first_index = digits.iter().position(|&r| r == first_index).unwrap();

    // find index of last digit from the digits array
    let mut last_index = digits.iter().position(|&r| r == last_index).unwrap();

    // if the position if the first digit is higher than ten, then subtract ten
    if first_index > 10 {
        first_index = first_index - 10;
    }

    // if the position if the last digit is higher than ten, then subtract ten
    if last_index > 10 {
        last_index = last_index - 10;
    }

    return (first_index as i32, last_index as i32);
}

// read a csv line per line from a path
fn read_csv() -> Result<i32, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_path("./data/day1/input.csv")?;

    // create empty array of numbers
    let mut numbers: Vec<i32> = Vec::new();

    for result in rdr.records() {
        let record = result?;

        let (first, last) = a_function(record[0].to_string());

        let concat = concat_numbers(first as i32, last as i32);

        numbers.push(concat.parse::<i32>().unwrap());
    }

    // add all numbers together
    let sum: i32 = numbers.iter().sum();

    Ok(sum)
}


fn main() {
    println!("Hello, world!");
    match read_csv() {
        Err(err) => eprintln!("error running example: {}", err),
        Ok(f) => println!("outcome: {}", f),
    }
}
