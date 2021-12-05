use super::common;

type BinVec = Vec<Vec<usize>>;

pub struct Solve;

impl Solve {
    pub fn riddle1(riddlefile: String) -> usize {
        let riddle_text = common::readfile(riddlefile.to_string());
        let riddle_vector = make_vec_from_string(riddle_text);
        calculate(riddle_vector)
    }

    pub fn riddle2(riddlefile: String) -> usize {
        let riddle_text = common::readfile(riddlefile.to_string());
        let riddle_vector = make_vec_from_string(riddle_text);
        calculate_gases(riddle_vector)
    }
}

fn make_vec_from_string(riddle_string: String) -> BinVec {
    let mut result_vec: BinVec = vec![vec![]];

    for s in riddle_string.lines() {
        let mut inner_vec = vec![];
        for x in s.chars() {
            match x {
                '0' => inner_vec.push(0),
                '1' => inner_vec.push(1),
                _ => {}
            };
        }
        result_vec.push(inner_vec);
    }
    result_vec
}

fn decimal(input: Vec<usize>) -> usize {
    let mut faktor = 1;
    let mut output = 0;
    let rev: Vec<usize> = input.into_iter().rev().collect();
    for i in rev.iter() {
        output += i * faktor;
        faktor *= 2;
    }
    output
}

fn calculate(input: BinVec) -> usize {
    // determine count of bits
    let mut bits = vec![];
    for _i in 0..input[1].len() {
        bits.push(vec![0, 0]);
    }
    for line in input.iter() {
        for x in 0..line.len() {
            bits[x][line[x]] += 1;
        }
    }
    // calculate gamma and epsilosn
    let mut gamma = vec![];
    let mut epsilon = vec![];
    for x in 0..bits.len() {
        if bits[x][0] > bits[x][1] {
            gamma.push(0);
            epsilon.push(1);
        } else {
            gamma.push(1);
            epsilon.push(0);
        }
    }
    let g = decimal(gamma);
    let e = decimal(epsilon);
    g * e
}

fn filter_by_criteria(mut input: BinVec, filter: usize, pos: usize) -> BinVec {
    let mut j = 0;
    while j < input.len() {
        if input[j][pos] != filter {
            input.swap_remove(j);
        } else {
            j += 1;
        }
    }
    input
}

fn make_counter_on_pos(input: BinVec, pos: usize) -> Vec<usize> {
    // init
    let mut bits = vec![0, 0];

    // count 0 and 1 on pos
    for line in input.iter() {
        bits[line[pos]] += 1;
    }

    // output
    bits
}

fn calculate_most(input: BinVec) -> usize {
    let mut pos = 0;
    let len = input[0].len();
    let mut data = input.clone();

    while pos < len {
        let bits = make_counter_on_pos(data.clone(), pos);
        let most = if bits[0] <= bits[1] { 1 } else { 0 };
        data = filter_by_criteria(data.clone(), most, pos);
        if data.len() == 1 {break};
        pos += 1;
    }
    decimal(data[0].clone())
}

fn calculate_least(input: BinVec) -> usize {
    let mut pos = 0;
    let len = input[0].len();
    let mut data = input.clone();
    
    while pos < len {
        let bits = make_counter_on_pos(data.clone(), pos);
        let least = if bits[0] <= bits[1] { 0 } else { 1 };
        data = filter_by_criteria(data.clone(), least, pos);
        if data.len() == 1 {break};
        pos += 1;
    }
    decimal(data[0].clone())
}

fn calculate_gases(input: BinVec) -> usize {
    // cleanup
    let mut data = input.clone();
    data.swap_remove(0);

    // get values
    let oxygen = calculate_most(data.clone());
    let co = calculate_least(data);
    // output
    oxygen * co
}

#[test]
fn riddle1() {
    let riddle_text = common::readfile("data/inputday03-test.txt".to_string());
    let riddle_vector = make_vec_from_string(riddle_text);
    let output = calculate(riddle_vector);
    assert_eq!(output, 198);
}

#[test]
fn riddle2() {
    let riddle_text = common::readfile("data/inputday03-test.txt".to_string());
    let riddle_vector = make_vec_from_string(riddle_text);
    let output = calculate_gases(riddle_vector);
    assert_eq!(output, 230);
}
