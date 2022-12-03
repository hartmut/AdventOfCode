use super::common;
pub type FoodVec = Vec<i64>;

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
        calculate_top_three(&mut riddle_vector)
    }
}
fn make_vec_from_string(riddle_string: String) -> FoodVec {
    let mut result_vec: FoodVec = vec![];

    for s in riddle_string.lines() {
        let without_whitespace = match s.split_whitespace().next() {
            None => 0,
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

fn calculate(input: &mut FoodVec) -> i64 {
    let mut maxfood: i64 = 0;
    let mut foodelf: i64 = 0;
    loop {
        match input.pop() {
            None => break,
            Some(x) => {
                if x == 0 {
                    if foodelf > maxfood {
                        maxfood = foodelf
                    }
                    foodelf = 0;
                } else {
                    foodelf += x;
                };
            }
        };
    }
    maxfood
}

fn calculate_top_three(input: &mut FoodVec) -> i64 {
    let mut maxfood= vec![0 as i64,0 as i64,0 as i64];
    let mut foodelf=0;
      loop {
        match input.pop() {
            None => break,
            Some(x) => {
                if x == 0 {
                    if foodelf > maxfood[0] {
                        maxfood[0] = foodelf;
                        maxfood.sort();
                    }
                    foodelf = 0;
                } else {
                    foodelf += x;
                };
            }
        };
    }
    maxfood.iter().sum()
}

#[test]
fn riddle1() {
    let riddle_text = common::readfile("data/inputday01-testdata.txt".to_string());
    let mut riddle_vector = make_vec_from_string(riddle_text);
    let output = calculate(&mut riddle_vector);
    assert_eq!(output, 24000);
}

#[test]
fn riddle2() {
    let riddle_text = common::readfile("data/inputday01-testdata.txt".to_string());
    let mut riddle_vector = make_vec_from_string(riddle_text);
    let output = calculate_top_three(&mut riddle_vector);
    assert_eq!(output, 45000);
}
