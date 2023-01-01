use crate::days::common::match_to_i64;

use super::common::{self};

pub struct Solve;

type RiddleData = String;

impl Solve {
    pub fn riddle1(riddlefile: String) -> i64 {
        let riddle_text = common::readfile(riddlefile.to_string());
        let mut riddledata = make_riddle_data(riddle_text);
        calculate(&mut riddledata)
    }

    pub fn riddle2(riddlefile: String) -> i64 {
        let riddle_text = common::readfile(riddlefile.to_string());
        let mut riddledata = make_riddle_data(riddle_text);
        calculate2(&mut riddledata)
    }
}

fn make_riddle_data(riddle_string: String) -> RiddleData {
    riddle_string
}

fn calculate(input: &mut RiddleData) -> i64 {
    let mut cycle = 1;
    let mut register = 1;
    let mut signal = 0;

    for l in input.lines() {
        if cycle % 40 == 20 {
            signal += register * cycle;
        }
        cycle += 1;
        let command = l.split_once(' ');
        if let Some(("addx", value)) = command {
            if cycle % 40 == 20 {
                signal += register * cycle;
            }
            register += match_to_i64(value.to_string());
            cycle += 1;
        }
    }

    signal
}

fn calculate2(input: &mut RiddleData) -> i64 {
    let mut cycle = 1;
    let mut register = 1;
    let mut signal = 0;

    for l in input.lines() {
        if cycle % 40 == 20 {
            signal += register * cycle;
        }
        cycle += 1;
        let command = l.split_once(' ');
        if let Some(("addx", value)) = command {
            if cycle % 40 == 20 {
                signal += register * cycle;
            }
            register += match_to_i64(value.to_string());
            cycle += 1;
        }
    }

    signal
}

#[test]
fn riddle1() {
    let riddle_text = common::readfile("data/inputday10-testdata.txt".to_string());
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate(&mut riddledata);
    assert_eq!(output, 13140);
}

#[test]
fn riddle2() {
    let riddle_text = common::readfile("data/inputday10-testdata.txt".to_string());
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate2(&mut riddledata);
    assert_eq!(output, 1);
}
