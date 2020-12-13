mod days;

use days::common::print_day_and_riddle;
use days::common::print_separator;

fn main() {
    println!("Hi, we will be going to the caribians this year and the story of the travel will be documented in this notebook https://adventofcode.com/2020/");
    print_separator();

    // day 1
    let input_file = "data/inputday01.txt";
    print_day_and_riddle(1, 1);
    println!(
        "in the expense report I found the two entries {:?}",
        days::day01::solve_day01_riddle1(input_file.to_string())
    );
    print_day_and_riddle(1, 2);
    println!(
        "in the expense report I found the two entries {:?}",
        days::day01::solve_day01_riddle2(input_file.to_string())
    );
    print_separator();
    // day 2
    let input_file = "data/inputday02.txt";
    print_day_and_riddle(2, 1);
    println!(
        "we have {:?} correct passwords",
        days::day02::solve_day02_riddle1(input_file.to_string())
    );
    print_day_and_riddle(2, 2);
    println!(
        "we have {:?} correct passwords using the second method ",
        days::day02::solve_day02_riddle2(input_file.to_string())
    );
    print_separator();
    // day 3
    let input_file = "data/inputday03.txt";
    print_day_and_riddle(3, 1);
    println!(
        "we would hit {:?} trees",
        days::day03::solve_day03_riddle1(input_file.to_string())
    );
    print_day_and_riddle(3, 2);
    println!(
        "{:?} trees I would hit? thats a lot ",
        days::day03::solve_day03_riddle2(input_file.to_string())
    );
    print_separator();
    // day 4
    let input_file = "data/inputday04.txt";
    print_day_and_riddle(4, 1);
    println!(
        "we do have {:?} correct passports",
        days::day04::solve_day04_riddle1(input_file.to_string())
    );
    print_day_and_riddle(4, 2);
    println!(
        "we do have {:?} checked passports",
        days::day04::solve_day04_riddle2(input_file.to_string())
    );
    print_separator();
    // day 5
    let input_file = "data/inputday05.txt";
    print_day_and_riddle(5, 1);
    println!(
        "the highst id on the boarding pass is {:?}.",
        days::day05::solve_day05_riddle1(input_file.to_string())
    );
    print_day_and_riddle(5, 2);
    println!(
        "my seat is {:?}",
        days::day05::solve_day05_riddle2(input_file.to_string())
    );
    print_separator();
    // day 6
    let input_file = "data/inputday05.txt";
    print_day_and_riddle(6, 1);
    println!(
        "the sum is {:?}.",
        days::day06::solve_riddle1(input_file.to_string())
    );
    print_day_and_riddle(6, 2);
    // println!(
    //     "my seat is {:?}",
    //     days::day06::solve_riddle2(input_file.to_string())
    // );
    print_separator();
}
