use super::common;

#[derive(Debug)]
struct Passwd {
    min: usize,
    max: usize,
    c: char,
    word: String,
}
pub type PassVec = Vec<Passwd>;

pub fn solve_day02_riddle1(riddlefile: String) -> usize {
    let riddle_text = common::readfile(riddlefile.to_string());
    let mut riddle_vector = make_vec_from_string(riddle_text);
    iterate(&mut riddle_vector)
}

pub fn solve_day02_riddle2(riddlefile: String) -> usize {
    let riddle_text = common::readfile(riddlefile.to_string());
    let mut riddle_vector = make_vec_from_string(riddle_text);
    iterate2(&mut riddle_vector)
}

fn make_vec_from_string(riddle_string: String) -> PassVec {
    let mut lines = riddle_string.lines();
    let mut result_vec: PassVec = vec![];

    for s in lines {
        let mut v: Vec<&str> = s.split(' ').collect();
        let passwd = v.pop().unwrap().to_string();
        let character = v.pop().unwrap().split(":").next().unwrap().chars().next().unwrap();
        let mut counter_iter = v.pop().unwrap().split("-");
        let pass_struct = Passwd{
            min: counter_iter.next().unwrap().parse::<usize>().unwrap(),
            max: counter_iter.next().unwrap().parse::<usize>().unwrap(),
            c: character,
            word: passwd,
        };
        result_vec.push(pass_struct);
    }
    result_vec
}

fn iterate(passwords: &mut PassVec) -> usize {
    let mut correct = 0;

    for line in passwords.iter(){
        let count =  line.word.matches(line.c).count();
        // println!("{:?}{:?} in {:?}", count, line.c, line.word);
        if count >= line.min && count <= line.max {correct += 1};
    }

    correct
}

fn iterate2(passwords: &mut PassVec) -> usize {
    let mut correct = 0;

    for line in passwords.iter(){
        let mut count = 0;
        let wordvec: Vec<_> = line.word.chars().collect();
        if wordvec[line.min-1] == line.c {count += 1}
        let wordvec: Vec<_> = line.word.chars().collect();
        if wordvec[line.max-1] == line.c {count += 1}
        if count == 1 {correct += 1}
    }

    correct
}
