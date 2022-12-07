use super::common::{self, charvec_to_digit, match_to_usize};

#[derive(Debug)]
struct SectionPair {
    elf1left: usize,
    elf1right: usize,
    elf2left: usize,
    elf2right: usize,
}

type PairVec = Vec<SectionPair>;

pub struct Solve;

impl Solve {
    pub fn riddle1(riddlefile: String) -> usize {
        let riddle_text = common::readfile(riddlefile.to_string());
        let mut riddle_vector = make_vec_from_string(riddle_text);
        calculate(&mut riddle_vector)
    }

    pub fn riddle2(riddlefile: String) -> usize {
        let riddle_text = common::readfile(riddlefile.to_string());
        let mut riddle_vector = make_vec_from_string(riddle_text);
        calculate2(&mut riddle_vector)
    }
}

fn make_vec_from_string(riddle_string: String) -> PairVec {
    let mut result_vec: PairVec = vec![];

    for s in riddle_string.lines() {
        let mut line = s.split_terminator(",");
        let mut elf1 = line.next().unwrap().split_terminator("-");
        let mut elf2 = line.next().unwrap().split_terminator("-");
        let elf1left = match_to_usize(elf1.next().unwrap().to_string());
        let elf1right = match_to_usize(elf1.next().unwrap().to_string());
        let elf2left = match_to_usize(elf2.next().unwrap().to_string());
        let elf2right = match_to_usize(elf2.next().unwrap().to_string());

        result_vec.push(SectionPair {
            elf1left,
            elf1right,
            elf2left,
            elf2right,
        });
    }
    result_vec
}

fn calculate(input: &mut PairVec) -> usize {
    let mut pairs: usize = 0;
    for pair in input.iter() {
        if ((pair.elf2left >= pair.elf1left) && (pair.elf2right <= pair.elf1right)
            || (pair.elf1left >= pair.elf2left) && (pair.elf1right <= pair.elf2right))
        {
            pairs += 1;
        };
    }

    pairs
}

fn calculate2(input: &mut PairVec) -> usize {
    let mut pairs: usize = 0;
    for pair in input.iter() {
        if ((pair.elf2left <= pair.elf1right ) && (pair.elf1left <= pair.elf2right))
        {
            pairs += 1;
        };
    }

    pairs
}


#[test]
fn riddle1() {
    let riddle_text = common::readfile("data/inputday04-testdata.txt".to_string());
    let mut riddle_vector = make_vec_from_string(riddle_text);
    let output = calculate(&mut riddle_vector);
    assert_eq!(output, 2);
}

#[test]
fn riddle2() {
    let riddle_text = common::readfile("data/inputday04-testdata.txt".to_string());
    let mut riddle_vector = make_vec_from_string(riddle_text);
    let output = calculate2(&mut riddle_vector);
    assert_eq!(output, 4);
}
