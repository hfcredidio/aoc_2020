#![feature(iterator_fold_self)]

use std::env;
use std::path::Path;

mod day1;
mod day2;
mod day3;
mod day5;
mod day6;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "1" => day1::day1(),
        "2" => day2::day2(&Path::new("data/day2.txt")),
        "3" => day3::day3(&Path::new("data/day3.txt")),
        "4" => println!("¯\\_(ツ)_/¯"),
        "5" => day5::day5(&Path::new("data/day5.txt")),
        "6" => day6::day6(&Path::new("data/day6.txt")),
        "7" => println!("¯\\_(ツ)_/¯"),
        _ => println!("This day is not available"),
    }
}
