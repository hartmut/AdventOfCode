use std::collections::btree_map::IntoIter;

use serde::de::value::UsizeDeserializer;

use crate::days::common::match_to_usize;

use super::common;

type BinVec = Vec<Vec<usize>>;

pub struct Solve;

impl Solve {
    pub fn riddle1(riddlefile: String) -> usize {
        let riddle_text = common::readfile(riddlefile.to_string());
        let riddle_vector = init_boards(riddle_text);
        // calculate(riddle_vector)
        0
    }

    // pub fn riddle2(riddlefile: String) -> usize {
    //     let riddle_text = common::readfile(riddlefile.to_string());
    //     let riddle_vector = make_vec_from_string(riddle_text);
    //     calculate_gases(riddle_vector)
    // }
}

#[derive(Debug)]
struct Board {
    numbers: [[usize; 5]; 5],
    marks: [[bool; 5]; 5],
}

// second Vec like Board with enum marked/unmarked
#[derive(Debug)]
struct Data {
    draws: Vec<usize>,
    boards: Vec<Board>,
}

fn create_draws_list(input: String) -> Vec<usize> {
    let mut draws: Vec<usize> = vec![];
    let mut splitdraws = input.split(',');
    for n in splitdraws {
        draws.push(match_to_usize(n.to_string()));
    }
    draws
}

fn init_a_board() -> Board {
    Board {
        numbers: [[0; 5]; 5],
        marks: [[false; 5]; 5],
    }
}

fn init_boards(riddle_string: String) -> Data {
    // init
    let boards = vec![];

    // get line iterator
    let mut iter = riddle_string.lines();

    // get all the draws and remove first empty line
    let draws = iter.next();
    let draws = create_draws_list(draws.unwrap().to_string());
    let mut y = 0;
    let _trash = iter.next();

    // and now read the boards

    let mut board = init_a_board();

    for s in iter {
        println!("{:?}", s);
        if s.len() == 0 {
            // reset board
            println!("empty");
            // insert board into output vector
            let mut board = init_a_board();
            y = 0;
        } else {
            // when there is data, import this into the board
            let mut split = s.split(' ');
            let mut x = 0;
            // TODO don't split at space but after three characters
            for n in split {
                // COMEBACK import data
                board.numbers[y][x] = match_to_usize(n.to_string());
                x += 1;
            }
            y += 1;
        };
    }

    // output
    Data { draws, boards }
}

#[test]
fn riddle1() {
    let riddle_text = common::readfile("data/inputday04-test.txt".to_string());
    let riddle_vector = init_boards(riddle_text);
    // let output = calculate(riddle_vector);
    // assert_eq!(output, 198);
}

#[test]
fn riddle2() {
    let riddle_text = common::readfile("data/inputday04-test.txt".to_string());
    let riddle_vector = init_boards(riddle_text);
    // let output = calculate_gases(riddle_vector);
    // assert_eq!(output, 230);
}
