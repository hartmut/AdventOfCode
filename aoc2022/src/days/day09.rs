use super::common::{self, match_to_usize};

pub struct Solve;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Pos {
    x: i16,
    y: i16,
}

type PosVisited = Vec<Pos>;

#[derive(Debug)]

enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Debug)]
struct Steps {
    dir: Direction,
    count: usize,
}

type RiddleData = Vec<Steps>;

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
    let mut output: RiddleData = vec![];

    for l in riddle_string.lines() {
        let (diri, counti) = l.rsplit_once(" ").unwrap();
        let mut dir = Direction::None;

        let count = match_to_usize(counti.to_string());
        match diri {
            "R" => dir = Direction::Right,
            "L" => dir = Direction::Left,
            "U" => dir = Direction::Up,
            "D" => dir = Direction::Down,
            _ => {}
        }

        let step = Steps { dir, count };

        output.push(step);
    }
    output
}

fn calculate(input: &mut RiddleData) -> usize {
    let mut score = 0;
    let mut visited: PosVisited = vec![];
    let mut posHead = Pos { x: 0, y: 0 };
    let mut posTail = Pos { x: 0, y: 0 };

    // iterate over steps
    for c in input.iter() {
        // iterate over counts
        for _ in 0..c.count {
            // println!("posTail: {:?}, posHead: {:?}, c: {:?}", posTail, posHead, c);
            match c.dir {
                Direction::Right => posHead.x += 1,
                Direction::Left => posHead.x += -1,
                Direction::Up => posHead.y += 1,
                Direction::Down => posHead.y -= 1,
                Direction::None => {}
            }

            // simulate Tail
            if (posTail.x - 1) > posHead.x {
                posTail.x -= 1;
                if posTail.y != posHead.y {
                    posTail.y = posHead.y;
                }
            };
            if (posTail.x + 1) < posHead.x {
                posTail.x += 1;
                if posTail.y != posHead.y {
                    posTail.y = posHead.y;
                }
            };
            if (posTail.y - 1) > posHead.y {
                posTail.y -= 1;
                if posTail.x != posHead.x {
                    posTail.x = posHead.x;
                }
            }
            if (posTail.y + 1) < posHead.y {
                posTail.y += 1;
                if posTail.x != posHead.x {
                    posTail.x = posHead.x;
                }
            };

            // println!("posTail: {:?}, posHead: {:?}", posTail, posHead);

            visited.push(posTail.clone());
        }
    }

    // println!("presort  : {:?}", visited);
    visited.sort();
    // println!("prededup : {:?}", visited);
    visited.dedup();
    // println!("postdedup: {:?}", visited);
    visited.len()
}

fn calculate2(input: &mut RiddleData) -> usize {
    let mut score = 0;

    score
}

#[test]
fn riddle1() {
    let riddle_text = common::readfile("data/inputday09-testdata.txt".to_string());
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate(&mut riddledata);
    assert_eq!(output, 13);
}

#[test]
fn riddle2() {
    let riddle_text = common::readfile("data/inputday09-testdata.txt".to_string());
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate2(&mut riddledata);
    assert_eq!(output, 1);
}
