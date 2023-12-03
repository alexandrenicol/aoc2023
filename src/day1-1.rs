use std::{error::Error};

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


// read a csv line per line from a path
fn read_csv() -> Result<i32, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_path("./data/day1/input.csv")?;

    // create empty array of numbers
    let mut numbers: Vec<i32> = Vec::new();

    for result in rdr.records() {
        let record = result?;

        let no_letters = remove_letters(&record[0]);
        let array = string_to_array(&no_letters);

        let first = array[0];
        let last = array[array.len() - 1];

        let concat = concat_numbers(first, last);

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
