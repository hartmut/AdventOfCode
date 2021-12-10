use super::common;

type BinVec = Vec<Vec<usize>>;

pub struct Solve;

impl Solve {
    pub fn riddle1(riddlefile: String) -> usize {
        let riddle_text = common::readfile(riddlefile.to_string());
        let riddle_vector = make_vec_from_string(riddle_text);
        // calculate(riddle_vector)
        0
    }

    // pub fn riddle2(riddlefile: String) -> usize {
    //     let riddle_text = common::readfile(riddlefile.to_string());
    //     let riddle_vector = make_vec_from_string(riddle_text);
    //     calculate_gases(riddle_vector)
    // }
}

type Board = [[usize; 5]; 5];

// second Vec like Board with enum marked/unmarked
struct Data {
    draws: Vec<usize>,
    boards: Vec<Board>,
}

fn create_draws_list(input: String) -> Vec<usize> {
    let draws: Vec<usize> = vec![];
    draws
}

fn make_vec_from_string(riddle_string: String) -> Data {
    let mut boards = vec![[[0;5];5]];

    let mut iter = riddle_string.lines();

    // get all the draws
    let draws = iter.next();
    let draws = create_draws_list(draws.unwrap().to_string());
    
    // COMEBACK import data
    // and now read the boards
    for s in riddle_string.lines() {}

    // clean up and create marked/unmarked Boards 
    boards.swap_remove(0);

    // output
    Data { draws, boards }
}

#[test]
fn riddle1() {
    let riddle_text = common::readfile("data/inputday04-test.txt".to_string());
    let riddle_vector = make_vec_from_string(riddle_text);
    // let output = calculate(riddle_vector);
    // assert_eq!(output, 198);
}

#[test]
fn riddle2() {
    let riddle_text = common::readfile("data/inputday04-test.txt".to_string());
    let riddle_vector = make_vec_from_string(riddle_text);
    // let output = calculate_gases(riddle_vector);
    // assert_eq!(output, 230);
}
