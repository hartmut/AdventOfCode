use super::common;
pub type DepthVec = Vec<i64>;

pub struct Solve;

impl Solve {
    pub fn riddle1(riddlefile: String) -> i64 {
        let riddle_text = common::readfile(riddlefile.to_string());
        let riddle_vector = make_vec_from_string(riddle_text);
        calculate(riddle_vector)
    }

    pub fn riddle2(riddlefile: String) -> i64 {
        let riddle_text = common::readfile(riddlefile.to_string());
        let riddle_vector = make_vec_from_string(riddle_text);
        calculate_sliding_window(riddle_vector)
    }
}
fn make_vec_from_string(riddle_string: String) -> DepthVec {
    let split = riddle_string.split("\n");
    let mut result_vec: DepthVec = vec![];

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

fn calculate(input: DepthVec) -> i64 {
    let len: usize = input.len();
    let mut output = 0;
    for x in 1..len {
        if input[x] > input[x - 1] {
            output += 1
        }
    }
    output
}

fn calculate_sliding_window(input: DepthVec) -> i64 {
    let len: usize = input.len();
    let mut output = 0;
    let mut x = 0;
    while x < (len - 3) {
        if (input[x] + input[x + 1] + input[x + 2]) < (input[x + 1] + input[x + 2] + input[x + 3]) {
            output += 1
        };
        x += 1;
    }
    output
}

#[test]
fn riddle1() {
    let v = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let i = calculate(v);
    assert_eq!(i, 7);
}

#[test]
fn riddle2() {
    let v = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let i = calculate_sliding_window(v);
    assert_eq!(i, 5);
}
