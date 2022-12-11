use super::common::{self};

pub struct Solve;

type RiddleData = String;

impl Solve {
    pub fn riddle1(riddlefile: String) -> usize {
        let riddle_text = common::readfile(riddlefile.to_string());
        let mut riddledata = make_riddle_data(riddle_text);
        calculate(&mut riddledata)
    }

    pub fn riddle2(riddlefile: String) -> usize {
        let riddle_text = common::readfile(riddlefile.to_string());
        let mut riddledata = make_riddle_data(riddle_text);
        calculate2(&mut riddledata)
    }
}

fn make_riddle_data(riddle_string: String) -> RiddleData {
    riddle_string
}

fn calculate(input: &mut RiddleData) -> usize {
    let mut inputvec = Vec::from(input.clone());
    let mut signal: Vec<u8> = inputvec.drain(..3).collect();
    let mut pos = 3;
    while inputvec.len() > 0 {
        let next: Vec<u8> = inputvec.drain(..1).collect();
        signal.push(next[0]);

        let mut test = signal.clone();
        test.sort();
        test.dedup();
        if test.len() == 4 {
            inputvec.clear();
        }
        let _ = signal.drain(..1);
        pos += 1;
    }
    pos
}

fn calculate2(input: &mut RiddleData) -> usize {
    let mut inputvec = Vec::from(input.clone());
    let mut pos = 13;
    let mut signal: Vec<u8> = inputvec.drain(..pos).collect();
    while inputvec.len() > 0 {
        let next: Vec<u8> = inputvec.drain(..1).collect();
        signal.push(next[0]);

        let mut test = signal.clone();
        test.sort();
        test.dedup();
        if test.len() == 14 {
            inputvec.clear();
        }
        let _ = signal.drain(..1);
        pos += 1;
    }
    pos
}

#[test]
fn riddle1() {
    let riddle_text = common::readfile("data/inputday06-testdata.txt".to_string());
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate(&mut riddledata);
    assert_eq!(output, 7);

    let riddle_text = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate(&mut riddledata);
    assert_eq!(output, 5);

    let riddle_text = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate(&mut riddledata);
    assert_eq!(output, 6);

    let riddle_text = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate(&mut riddledata);
    assert_eq!(output, 10);

    let riddle_text = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate(&mut riddledata);
    assert_eq!(output, 11);
}

#[test]
fn riddle2() {
    let riddle_text = common::readfile("data/inputday06-testdata.txt".to_string());
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate2(&mut riddledata);
    assert_eq!(output, 19);

    let riddle_text = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate2(&mut riddledata);
    assert_eq!(output, 23);

    let riddle_text = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate2(&mut riddledata);
    assert_eq!(output, 23);

    let riddle_text = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate2(&mut riddledata);
    assert_eq!(output, 29);

    let riddle_text = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate2(&mut riddledata);
    assert_eq!(output, 26);
}
