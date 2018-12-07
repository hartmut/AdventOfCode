mod days;

use days::common::print_separator;

fn main() {
    println!("Hi, we are sliding through time. You will find the story in this notebook https://adventofcode.com/2018/");
    print_separator();

    // day 1
    let input_file = "data/inputday01.txt";
    println!("we are on day 1",);
    println!(
        "the resulting frequency is {:?}",
        days::day01::solve_day01_riddle1(input_file.to_string())
    );
    println!(
        "the frequency {:?} is the first we see two times",
        days::day01::solve_day01_riddle2(input_file.to_string())
    );
    print_separator();
    // day 2
    let input_file = "data/inputday02.txt";
    println!("we are on day 2",);
    println!(
        "the checksum is {:?}",
        days::day02::solve_day02_riddle1(input_file.to_string())
    );
    println!(
        "the crates have the ID {:?}",
        days::day02::solve_day02_riddle2(input_file.to_string())
    );
    print_separator();
    // day 3
    let input_file = "data/inputday03.txt";
    println!("we are on day 3",);
    println!(
        "the checksum is {:?}",
        days::day03::solve_day03_riddle1(input_file.to_string())
    );
    print_separator();
}
