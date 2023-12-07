mod helpers;
mod days;

use days::day4::day4_1;

fn main() {
    println!("Hello, world!");
    match day4_1() {
        Err(err) => eprintln!("error running example: {}", err),
        Ok(f) => println!("outcome: {}", f),
    }
}
