use super::common::{self, match_to_usize};

type Stacks = Vec<String>;

#[derive(Debug)]
struct Rule {
    count: usize,
    start: usize,
    to: usize,
}

type Rules = Vec<Rule>;

pub struct Solve;

#[derive(Debug)]
struct RiddleData {
    stacks: Vec<Stacks>,
    rules: Rules,
}

impl Solve {
    pub fn riddle1(riddlefile: String) -> String {
        let riddle_text = common::readfile(riddlefile.to_string());
        let mut riddledata = make_riddle_data(riddle_text);
        calculate(&mut riddledata)
    }

    pub fn riddle2(riddlefile: String) -> String {
        let riddle_text = common::readfile(riddlefile.to_string());
        let mut riddledata = make_riddle_data(riddle_text);
        calculate2(&mut riddledata)
    }
}

fn make_riddle_data(riddle_string: String) -> RiddleData {
    let mut result = RiddleData {
        stacks: vec![],
        rules: vec![],
    };
    let mut input_iter = riddle_string.lines();

    // build stacks
    // find entry point and save stacks in inverse order
    let mut inverse_stack: Vec<String> = vec![];
    loop {
        let l = input_iter.next().unwrap().to_string();
        if l.len() == 0 {
            break;
        } else {
            inverse_stack.push(l);
        };
    }

    // create stack data
    // find count of stacks
    let c = inverse_stack
        .pop()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .to_string();
    let c = match_to_usize(c);
    for _ in 0..c {
        let one_stack = vec![];
        result.stacks.push(one_stack);
    }

    // insert data into the stacks
    while inverse_stack.len() > 0 {
        let s = inverse_stack.pop().unwrap();
        let (mut left, mut right) = s.split_at(4);
        // iterate over every line and insert the values into the stacks
        for i in 0..c {
            let element = left.to_string();
            let element = element.get(1..2).unwrap().to_ascii_uppercase();
            if element != " " {
                result.stacks[i].push(element);
            }
            if right.len() > 3 {
                (left, right) = right.split_at(4);
            } else {
                left = right;
            }
        }
    }
    // read rules

    while let Some(l) = input_iter.next() {
        // split at whitelines
        let mut l_iter = l.split_whitespace();
        let _drop = l_iter.next();
        let count = match_to_usize(l_iter.next().unwrap().to_string());
        let _drop = l_iter.next();
        let start = match_to_usize(l_iter.next().unwrap().to_string());
        let _drop = l_iter.next();
        let to = match_to_usize(l_iter.next().unwrap().to_string());
        result.rules.push(Rule { count, start, to });
    }

    result
}

fn calculate(input: &mut RiddleData) -> String {
    for iter in input.rules.iter() {
        for _i in 0..iter.count {
            let mv = input.stacks[iter.start - 1].pop().unwrap();
            input.stacks[iter.to - 1].push(mv);
        }
    }

    let mut output = "".to_string();
    for i in 0..input.stacks.len() {
        output.push_str(&input.stacks[i].pop().unwrap());
    }

    output
}

fn calculate2(input: &mut RiddleData) -> String {
    for iter in input.rules.iter() {
        let mut puffer: Vec<String> = vec![];
        for _i in 0..iter.count {
            let mv = input.stacks[iter.start-1].pop().unwrap();
            puffer.push(mv);
        }

        while puffer.len()>0 {
            input.stacks[iter.to-1].push(puffer.pop().unwrap());
        }

    }

    let mut output = "".to_string();
    for i in 0..input.stacks.len() {
        output.push_str(&input.stacks[i].pop().unwrap());
    }

    output
}

#[test]
fn riddle1() {
    let riddle_text = common::readfile("data/inputday05-testdata.txt".to_string());
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate(&mut riddledata);
    assert_eq!(output, "CMZ".to_string());
}

#[test]
fn riddle2() {
    let riddle_text = common::readfile("data/inputday05-testdata.txt".to_string());
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate2(&mut riddledata);
    assert_eq!(output, "MCD".to_string());
}
