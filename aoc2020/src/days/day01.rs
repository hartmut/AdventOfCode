use super::common;
use std::collections::HashSet;
pub type ExpenseVec = Vec<i64>;

pub fn solve_day01_riddle1(riddlefile: String) -> i64 {
    let riddle_text = common::readfile(riddlefile.to_string());
    let riddle_vector = make_vec_from_string(riddle_text);
    calculate(riddle_vector)
    // calculate(riddle_vector)
}

pub fn solve_day01_riddle2(riddlefile: String) -> i64 {
    let riddle_text = common::readfile(riddlefile.to_string());
    let riddle_vector = make_vec_from_string(riddle_text);
    calculate_three(riddle_vector)
}

fn make_vec_from_string(riddle_string: String) -> ExpenseVec {
    let split = riddle_string.split("\n");
    let mut result_vec: ExpenseVec = vec![];

    for s in split {
        let without_whitespace = match s.split_whitespace().next() {
            None => break,
            Some(x) => match x.parse::<i64>() {
                Err(x) => {
                    println!("{:?}", x);
                    break;
                }
                Ok(x) => x,
            },
        };
        result_vec.push(without_whitespace);
    }
    result_vec
}

fn calculate(input: ExpenseVec) -> i64 {
    let len: usize = input.len();
    let mut output = 0;
    'outer: for x in (0..len) {
        for y in (0..len) {
            if input[x] + input[y] == 2020 {
                output = input[x] * input[y] ;
            }
        }
    }
    output
}

fn calculate_three(input: ExpenseVec) -> i64 {
    let len: usize = input.len();
    let mut output = 0;
    'outer: for x in (0..len) {
        for y in (0..len) {
            for z in (0..len) {
            if input[x] + input[y] + input[z]== 2020 {
                output = input[x] * input[y] *input[z];
            }
        }}
    }
    output
}
