mod days;

use crate::days::common::*;

// TODO build filename of inputfile from day and standardtext

fn main() {
    let mut day = 1;
    println!("Oh no, we lost the key https://adventofcode.com/2022/");
    print_separator();

    // day 1
    let input_file = "data/inputday01.txt";
    print_day_and_riddle(day, 1);
    println!(
        "how much food carries the elf with the most food? {:?}",
        days::day01::Solve::riddle1(input_file.to_string())
    );
    print_day_and_riddle(day, 2);
    println!(
        "how much food carries the three elfs with the most food? {:?}",
        days::day01::Solve::riddle2(input_file.to_string())
    );
    print_separator();

    // day 2
    day += 1;
    let input_file = "data/inputday02.txt";
    print_day_and_riddle(day, 1);
    println!(
        "what is my total score? {:?}",
        days::day02::Solve::riddle1(input_file.to_string())
    );
    print_day_and_riddle(day, 2);
    println!(
        "what is my total score with the new strategy? {:?}",
        days::day02::Solve::riddle2(input_file.to_string())
    );
    print_separator();

    // day 3
    day += 1;
    let input_file = "data/inputday03.txt";
    print_day_and_riddle(day, 1);
    println!(
        "What is the sum of the priorities of the item types? {:?}",
        days::day03::Solve::riddle1(input_file.to_string())
    );
    print_day_and_riddle(day, 2);
    println!(
        "What is the sum of the priorities of the searched badge item types? {:?}",
        days::day03::Solve::riddle2(input_file.to_string())
    );
    print_separator();

    // day 4
    day += 1;
    print_day_and_riddle(day, 1);
    let input_file = "data/inputday04.txt";
    println!(
        "In how many assignment pairs does one range fully contain the other? {:?}",
        days::day04::Solve::riddle1(input_file.to_string())
    );
    print_day_and_riddle(day, 2);
    println!(
        "In how many assignment pairs do the ranges overlap? {:?}",
        days::day04::Solve::riddle2(input_file.to_string())
    );
    print_separator();

    // day 5
    day += 1;
    let input_file = "data/inputday05.txt";
    print_day_and_riddle(day, 1);
    println!(
        "After the rearrangement procedure completes, what crate ends up on top of each stack? {:?}",
        days::day05::Solve::riddle1(input_file.to_string())
    );
    print_day_and_riddle(day, 2);
    println!(
        "In how many assignment pairs do the ranges overlap? {:?}",
        days::day05::Solve::riddle2(input_file.to_string())
    );
    print_separator();

    // day 6
    day += 1;
    let input_file = "data/inputday06.txt";
    print_day_and_riddle(day, 1);
    println!(
            "After the rearrangement procedure completes, what crate ends up on top of each stack? {:?}",
            days::day06::Solve::riddle1(input_file.to_string())
        );
    print_day_and_riddle(day, 2);
    println!(
        "How many characters need to be processed before the first start-of-message marker is detected? {:?}",
        days::day06::Solve::riddle2(input_file.to_string())
    );
    print_separator();
}
