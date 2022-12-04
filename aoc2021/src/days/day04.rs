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
    //     calculate2(riddle_vector)
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
    // output
    Data { draws, boards }
}

fn update_board_and_check(board: &mut Board, value: usize) -> bool {
    // update a board
    let mut done = false;
    for row in 0..5 {
        for line in 0..5 {
            if board.numbers[row][line] == value {
                board.marks[row][line] = true;
                done = check_board_on_bingo(board, row, line);
                break;
            }
        }
        if done {
            break;
        };
    }
    done
}

fn check_board_on_bingo(board: &mut Board, rowin: usize, linein: usize) -> bool {
    let mut checkrow = false;
    let mut checkline = false;
    for row in 0..5 {
        checkline = board.marks[row][linein];
        if !checkline {
            break;
        };
    }
    for line in 0..5 {
        checkrow = board.marks[rowin][line];
        if !checkrow {
            break;
        };
    }

    checkrow | checkline
}

fn calculate_board_value(board: &mut Board) -> usize {
    let mut score = 0;
    for row in 0..5 {
        for line in 0..5 {
            if !board.marks[row][line] {
                score += board.numbers[row][line];
            }
        }
    }
    score
}

fn calculate(data: &mut Data) -> usize {
    let mut iter = data.draws.iter();
    let mut done = false;
    let mut score = 0;

    
    // iterate over all draws and break if the last one gets you to a bingo
    for draw in iter {
        let mut found = false;
        // iterate over all boards
        for b in 0..data.boards.len() {
            // check whether one board has a bingo
            if update_board_and_check(&mut data.boards[b], *draw) {
                score = calculate_board_value(&mut data.boards[b]) * draw;
                found = true;
                break;
            }
        }
        if found {
            break;
        };
    }

    score
}

fn calculate2(data: &mut Data) -> usize {
    let mut iter = data.draws.iter();
    let mut done = false;
    let mut score = 0;
    let mut lastboard = 0;
    let mut lastdraw = 0;
    
    // iterate over all draws and break if the last one gets you to a bingo
    for draw in iter {
        // iterate over all boards
        for b in 0..data.boards.len() {
            // check whether one board has a bingo
            if update_board_and_check(&mut data.boards[b], *draw) {
                lastboard = b;
                lastdraw = *draw;
                break;
            }
        }
    }
    calculate_board_value(&mut data.boards[lastboard]) * lastdraw
}

#[test]
fn riddle1() {
    let riddle_text = common::readfile("data/inputday04-test.txt".to_string());
    let mut riddle_vector = init_boards(riddle_text);
    let output = calculate(&mut riddle_vector);
    assert_eq!(output, 4512);
}

#[test]
fn riddle2() {
    let riddle_text = common::readfile("data/inputday04-test.txt".to_string());
    let mut riddle_vector = init_boards(riddle_text);
    let output = calculate2(&mut riddle_vector);
    assert_eq!(output, 1924);
}
