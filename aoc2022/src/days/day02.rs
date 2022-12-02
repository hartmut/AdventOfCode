use super::common;

#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

// in riddle 2 I need to translate my own values (i)
// X (Rock) means you need to lose
// Y (Paper) means you need to end the round in a draw and
// Z (Scissor) means you need to win.
#[derive(Debug)]
struct Round {
    elf: Hand,
    i: Hand,
}

type GameVec = Vec<Round>;

pub struct Solve;

impl Solve {
    pub fn riddle1(riddlefile: String) -> i64 {
        let riddle_text = common::readfile(riddlefile.to_string());
        let mut riddle_vector = make_vec_from_string(riddle_text);
        calculate(&mut riddle_vector)
    }

    pub fn riddle2(riddlefile: String) -> i64 {
        let riddle_text = common::readfile(riddlefile.to_string());
        let mut riddle_vector = make_vec_from_string(riddle_text);
        calculate_translation(&mut riddle_vector)
    }
}

fn make_vec_from_string(riddle_string: String) -> GameVec {
    let mut result_vec: GameVec = vec![];

    for s in riddle_string.lines() {
        let mut onegame = Round {
            elf: Hand::Rock,
            i: Hand::Rock,
        };
        let mut c = s.split_whitespace();
        let elf = c.next().unwrap();
        match elf {
            "A" => onegame.elf = Hand::Rock,
            "B" => onegame.elf = Hand::Paper,
            "C" => onegame.elf = Hand::Scissor,
            _ => break,
        }
        let i = c.next().unwrap();
        match i {
            "X" => onegame.i = Hand::Rock,
            "Y" => onegame.i = Hand::Paper,
            "Z" => onegame.i = Hand::Scissor,
            _ => break,
        }

        result_vec.push(onegame);
    }
    result_vec
}

// X (Rock) means you need to lose
// Y (Paper) means you need to end the round in a draw and
// Z (Scissor) means you need to win.
fn calculate_translation(input: &mut GameVec) -> i64 {
    let mut score: i64 = 0;
    for round in input.iter() {
        match round.elf {
            Hand::Rock => match round.i {
                Hand::Rock => score += 3,    //loose - 3 for scissor and 0 for loose
                Hand::Paper => score += 4,   //draw - 1 for rock and 3 for draw
                Hand::Scissor => score += 8, //win - 2 for paper and 6 for win
            },
            Hand::Paper => match round.i {
                Hand::Rock => score += 1,    //loose - 1 for rock and 0 for loose
                Hand::Paper => score += 5,   //draw - 2 for paper and 3 for draw
                Hand::Scissor => score += 9, //win - 3 for scissor and 6 for win
            },
            Hand::Scissor => match round.i {
                Hand::Rock => score += 2,    //loose - 2 for paper and 0 for loose
                Hand::Paper => score += 6,   //draw - 3 for scissor and 3 for draw
                Hand::Scissor => score += 7, //win - 1 for rock and 6 for win
            },
        };
    }
    score
}

fn calculate(input: &mut GameVec) -> i64 {
    let mut score: i64 = 0;
    for round in input.iter() {
        // calculate shape value
        match round.i {
            Hand::Rock => score += 1,
            Hand::Paper => score += 2,
            Hand::Scissor => score += 3,
        }
        // calculate round winner
        match round.elf {
            Hand::Rock => match round.i {
                Hand::Rock => score += 3,
                Hand::Paper => score += 6,
                Hand::Scissor => score += 0,
            },
            Hand::Paper => match round.i {
                Hand::Rock => score += 0,
                Hand::Paper => score += 3,
                Hand::Scissor => score += 6,
            },
            Hand::Scissor => match round.i {
                Hand::Rock => score += 6,
                Hand::Paper => score += 0,
                Hand::Scissor => score += 3,
            },
        }
    }
    score
}

#[test]
fn riddle1() {
    let riddle_text = common::readfile("data/inputday02-testdata.txt".to_string());
    let mut riddle_vector = make_vec_from_string(riddle_text);
    let output = calculate(&mut riddle_vector);
    assert_eq!(output, 15);
}

#[test]
fn riddle2() {
    let riddle_text = common::readfile("data/inputday02-testdata.txt".to_string());
    let mut riddle_vector = make_vec_from_string(riddle_text);
    let output = calculate_translation(&mut riddle_vector);
    assert_eq!(output, 12);
}
