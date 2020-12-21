#![feature(iterator_fold_self)]
#![feature(peekable_next_if)]

use std::env;
use std::path::Path;

mod day1;
mod day10;
mod day12;
mod day13;
mod day2;
mod day3;
mod day5;
mod day6;
mod day8;
mod day9;

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
        "8" => day8::day8(&Path::new("data/day8.txt")),
        "9" => day9::day9(&Path::new("data/day9.txt")),
        "10" => day10::day10(&Path::new("data/day10.txt")),
        "11" => println!("¯\\_(ツ)_/¯"),
        "12" => day12::day12(&Path::new("data/day12.txt")),
        "13" => day13::day13(),
        _ => println!("This day is not available"),
    }
}
