mod days;

use days::common::print_day_and_riddle;
use days::common::print_separator;

fn main() {
    println!("Hi, we are going on a rocket trip. You will find the story in this notebook https://adventofcode.com/2019/");
    print_separator();

    // day 1
    let input_file = "data/inputday01.txt";
    print_day_and_riddle(1, 1);
    println!(
        "we need {:?} fuel",
        days::day01::solve_day01_riddle1(input_file.to_string())
    );
    println!(
        "but becaus of the rocket equation in reality we do need {:?} fuel",
        days::day01::solve_day01_riddle2(input_file.to_string())
    );
    print_separator();
}
