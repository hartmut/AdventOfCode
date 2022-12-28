mod days;

use crate::days::common::*;

fn main() {
    let mut day = 1;
    println!("We need star fruits https://adventofcode.com/2022/");

    // day 1
    let input_file = datafile(day);
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

    // day 2
    day += 1;
    let input_file = datafile(day);
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

    // day 3
    day += 1;
    let input_file = datafile(day);
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

    // day 4
    day += 1;
    print_day_and_riddle(day, 1);
    let input_file = datafile(day);
    println!(
        "In how many assignment pairs does one range fully contain the other? {:?}",
        days::day04::Solve::riddle1(input_file.to_string())
    );
    print_day_and_riddle(day, 2);
    println!(
        "In how many assignment pairs do the ranges overlap? {:?}",
        days::day04::Solve::riddle2(input_file.to_string())
    );

    // day 5
    day += 1;
    let input_file = datafile(day);
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

    // day 6
    day += 1;
    let input_file = datafile(day);
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

    // day 7
    day += 1;
    let input_file = datafile(day);
    print_day_and_riddle(day, 1);
    println!(
                "After the rearrangement procedure completes, what crate ends up on top of each stack? {:?}",
                days::day07::Solve::riddle1(input_file.to_string())
            );
    print_day_and_riddle(day, 2);
    println!(
        "What is the total size of the directory I need to delete to free up enough space? {:?}",
        days::day07::Solve::riddle2(input_file.to_string())
    );

    // day 8
    day += 1;
    let input_file = datafile(day);
    print_day_and_riddle(day, 1);
    println!(
        "how many trees are visible from outside the grid? {:?}",
        days::day08::Solve::riddle1(input_file.to_string())
    );
    print_day_and_riddle(day, 2);
    println!(
        "What is the highest scenic score possible for any tree? {:?}",
        days::day08::Solve::riddle2(input_file.to_string())
    );

    // day 9
    day += 1;
    let input_file = datafile(day);
    print_day_and_riddle(day, 1);
    println!(
        "How many positions does the tail of the rope visit at least once? {:?}",
        days::day09::Solve::riddle1(input_file.to_string())
    );
    print_day_and_riddle(day, 2);
    println!(
        "How many positions does the tail of the rope visit at least once? {:?}",
        days::day09::Solve::riddle2(input_file.to_string())
    );
}
