use std::env;
use std::path::Path;

mod day1;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "1" => day1::day1(),
        "2" => day2::day2(&Path::new("data/day2.txt")),
        "3" => day3::day3(&Path::new("data/day3.txt")),
        _ => println!("This day is not available"),
    }
}
