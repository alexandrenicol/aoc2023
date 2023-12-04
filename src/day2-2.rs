use std::{error::Error};
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}


// read a csv line per line from a path
fn read_csv() -> Result<i32, Box<dyn Error>> {
    let lines = read_lines("./data/day2/input.csv");

    // create empty array of numbers
    let mut numbers: Vec<i32> = Vec::new();

    for row in lines {
        //make a copy of line
        let mut line = row.clone();

        // remove "Game " from the line
        line = line.replace("Game ", "");

        // split by colon
        let split: Vec<&str> = line.split(":").collect();

        let id = split[0].parse::<i32>().unwrap();
        let scores = split[1].replace(";", ",");

        let balls: Vec<&str> = scores.split(",").collect();

        let mut min_green = 0;
        let mut min_blue = 0;
        let mut min_red = 0;

        for ball in balls {
            let data: Vec<&str> = ball.trim().split(" ").collect();
            let number = data[0].parse::<i32>().unwrap();
            let colour = data[1];

            if colour == "red" && number > min_red {
                min_red = number;
            }
            if colour == "green" && number > min_green {
                min_green = number;
            }
            if colour == "blue" && number > min_blue {
                min_blue = number;
            }
        }

        numbers.push(min_red * min_green * min_blue);

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
