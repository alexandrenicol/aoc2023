mod helpers;
mod days;

use days::day3::day3_2;

fn main() {
    println!("Hello, world!");
    match day3_2() {
        Err(err) => eprintln!("error running example: {}", err),
        Ok(f) => println!("outcome: {}", f),
    }
}
