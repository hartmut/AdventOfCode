mod days;

use crate::days::common::*;

fn main() {
    println!("Oh no, we lost the key https://adventofcode.com/2021/");
    print_separator();

    // day 1
    let input_file = "data/inputday01.txt";
    print_day_and_riddle(1, 1);
    println!(
        "how fast does the sea bed sink? {:?}",
        days::day01::Solve::riddle1(input_file.to_string())
    );
    print_day_and_riddle(1, 2);
    println!(
        "ok, we need to remove all the noise from the data, so how fast does it sink now? {:?}",
        days::day01::Solve::riddle2(input_file.to_string())
    );
    print_separator();
    // day 2
    let input_file = "data/inputday02.txt";
    print_day_and_riddle(2, 1);
    println!(
        "so where are we? {:?}",
        days::day02::Solve::riddle1(input_file.to_string())
    );
    print_day_and_riddle(2, 2);
    println!(
    "my aim seems slightly off {:?}",
    days::day02::Solve::riddle2(input_file.to_string())
    );
    print_separator();
}
