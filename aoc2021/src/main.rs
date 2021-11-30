mod days;

use crate::days::common::*;

fn main() {
    println!("Hi, we will be going to the caribians this year and the story of the travel will be documented in this notebook https://adventofcode.com/2021/");
    print_separator();

    // day 1
    let input_file = "data/inputday01.txt";
    print_day_and_riddle(1, 1);
    println!(
        "in the expense report I found the two entries {:?}",
        days::day01::Solve::riddle1(input_file.to_string())
    );
    print_day_and_riddle(1, 2);
    println!(
        "in the expense report I found the two entries {:?}",
        days::day01::Solve::riddle2(input_file.to_string())
    );
    print_separator();
   
}
