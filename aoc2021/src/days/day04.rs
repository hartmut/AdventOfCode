use crate::days::common::match_to_usize;

use super::common;
pub struct Solve;

impl Solve {
    pub fn riddle1(riddlefile: String) -> usize {
        let riddle_text = common::readfile(riddlefile.to_string());
        let mut riddle_vector = init_boards(riddle_text);
        calculate(&mut riddle_vector)
    }

    // pub fn riddle2(riddlefile: String) -> usize {
    //     let riddle_text = common::readfile(riddlefile.to_string());
    //     let riddle_vector = make_vec_from_string(riddle_text);
    //     calculate_gases(riddle_vector)
    // }
}

#[derive(Debug, Clone)]
struct Board {
    numbers: [[usize; 5]; 5],
    marks: [[bool; 5]; 5],
}

// second Vec like Board with enum marked/unmarked
#[derive(Debug, Clone)]
struct Data {
    draws: Vec<usize>,
    boards: Vec<Board>,
}

fn create_draws_list(input: String) -> Vec<usize> {
    let mut draws: Vec<usize> = vec![];
    let splitdraws = input.split(',');
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
    let mut boards = vec![];

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
        if s.len() == 0 {
            // insert board into output vector
            boards.push(board.clone());
            // reset board
            board = init_a_board();
            y = 0;
        } else {
            // when there is data, import this into the board
            let split = s.split(' ');
            let mut x = 0;
            for n in split {
                // if there is a one digit number we get an entry with no value because of two spaces as separators
                if n.len() > 0 {
                    board.numbers[y][x] = match_to_usize(n.to_string());
                    x += 1;
                };
            }
            y += 1;
        }
    }
    boards.push(board.clone());
    println!("{:?}", boards);
    println!("{:?}", draws);
    // output
    Data { draws, boards }
}

fn update_all_boards(boards: &mut Vec<Board>, value: usize) {
// COMEBACK update all boards
}

fn calculate(data: &mut Data) -> usize {
    let mut iter = data.draws.iter();
    let mut done = false;
    let mut value: usize = 0;
    let mut bingo_board = -1;

    while !done {
        let draw = iter.next();
        match draw {
            None => {
                done = true;
                break;
            }
            Some(value) => {
                // iterate over all boards
                update_all_boards(&mut data.boards, *value);
                // check whether one board has a bingo
                // break if yes and communicate bingo board number 
            },
        }
    }
    // calculate value for bingo Board using variable bingo_board

    0
}

#[test]
fn riddle1() {
    let riddle_text = common::readfile("data/inputday04-test.txt".to_string());
    let _riddle_vector = init_boards(riddle_text);
    // let output = calculate(riddle_vector);
    // assert_eq!(output, 198);
}

#[test]
fn riddle2() {
    let riddle_text = common::readfile("data/inputday04-test.txt".to_string());
    let _riddle_vector = init_boards(riddle_text);
    // let output = calculate_gases(riddle_vector);
    // assert_eq!(output, 230);
}
