use super::common::{self, charvec_to_digit};

struct Line {
    left: String,
    right: String,
}

type BackpackVec = Vec<Line>;

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
        calculate_badge_types(&mut riddle_vector)
    }
}

fn make_vec_from_string(riddle_string: String) -> BackpackVec {
    let mut result_vec: BackpackVec = vec![];

    for s in riddle_string.lines() {
        let x = s.len() / 2;
        let (left, right) = s.split_at(x);

        result_vec.push(Line {
            left: left.to_string(),
            right: right.to_string(),
        });
    }
    result_vec
}

fn correct_priority(invec: Vec<i64>) -> Vec<i64> {
    let mut outvec: Vec<i64> = vec![];
    for i in invec.iter() {
        if *i > 96 {
            outvec.push(i - 96)
        };
        if *i < 96 {
            outvec.push(i - 64 + 26)
        };
    }
    outvec
}

fn calculate(input: &mut BackpackVec) -> i64 {
    let mut score: i64 = 0;
    for line in input.iter() {
        // prepare the data
        let mut left = correct_priority(charvec_to_digit(line.left.chars().collect()));
        let mut right = correct_priority(charvec_to_digit(line.right.chars().collect()));
        left.sort();
        left.dedup();
        right.sort();
        right.dedup();

        // find the item in both compartments
        for x in left.iter() {
            let mut found = false;
            for y in 0..right.len() {
                if *x == right[y] {
                    score += *x;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            };
        }
    }

    score
}

fn calculate_badge_types(input: &mut BackpackVec) -> i64 {
    let mut score: i64 = 0;
    let mut input_iter = input.iter();
    loop {
        match input_iter.next() {
            None => break,
            Some(line) => {
                // first team member
                let mut first = correct_priority(charvec_to_digit(line.left.chars().collect()));
                let mut right = correct_priority(charvec_to_digit(line.right.chars().collect()));
                first.append(&mut right);
                first.sort();
                first.dedup();

                // second team member
                let line = input_iter.next().unwrap();
                let mut second = correct_priority(charvec_to_digit(line.left.chars().collect()));
                let mut right = correct_priority(charvec_to_digit(line.right.chars().collect()));
                second.append(&mut right);
                second.sort();
                second.dedup();

                // third team member
                let line = input_iter.next().unwrap();
                let mut third = correct_priority(charvec_to_digit(line.left.chars().collect()));
                let mut right = correct_priority(charvec_to_digit(line.right.chars().collect()));
                third.append(&mut right);
                third.sort();
                third.dedup();

                // find the item in both compartments
                for x in first.iter() {
                    let mut found = false;
                    let mut found_first = false;

                    for y in 0..second.len() {
                        if *x == second[y] {
                            found_first = true;
                            break;
                        }
                    }

                    for y in 0..third.len() {
                        if *x == third[y] {
                            if found_first {
                                score += *x;
                                found = true;
                            }
                            break;
                        }
                    }
                    if found {
                        break;
                    };
                }
            }
        }
    }

    score
}

#[test]
fn riddle1() {
    let riddle_text = common::readfile("data/inputday03-testdata.txt".to_string());
    let mut riddle_vector = make_vec_from_string(riddle_text);
    let output = calculate(&mut riddle_vector);
    assert_eq!(output, 157);
}

#[test]
fn riddle2() {
    let riddle_text = common::readfile("data/inputday03-testdata.txt".to_string());
    let mut riddle_vector = make_vec_from_string(riddle_text);
    let output = calculate_badge_types(&mut riddle_vector);
    assert_eq!(output, 70);
}
