mod days;

use crate::days::common::*;

fn main() {
    println!("Oh no, we lost the key https://adventofcode.com/2021/");
    print_separator();

    // day 1
    let input_file = "data/inputday01.txt";
    print_day_and_riddle(1, 1);
    println!(
        "how much food carries the elf with the most food? {:?}",
        days::day01::Solve::riddle1(input_file.to_string())
    );
    print_day_and_riddle(1, 2);
    println!(
        "how much food carries the three elfs with the most food? {:?}",
        days::day01::Solve::riddle2(input_file.to_string())
    );
    print_separator();

    // day 2
    let input_file = "data/inputday02.txt";
    print_day_and_riddle(2, 1);
    println!(
        "how much food carries the elf with the most food? {:?}",
        days::day02::Solve::riddle1(input_file.to_string())
    );
}
