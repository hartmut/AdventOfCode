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
    let mut visited: PosVisited = vec![];
    let mut pos_head = Pos { x: 0, y: 0 };
    let mut pos_tail = Pos { x: 0, y: 0 };

    // iterate over steps
    for c in input.iter() {
        // iterate over counts
        for _ in 0..c.count {
            match c.dir {
                Direction::Right => pos_head.x += 1,
                Direction::Left => pos_head.x += -1,
                Direction::Up => pos_head.y += 1,
                Direction::Down => pos_head.y -= 1,
                Direction::None => {}
            }

            // simulate Tail
            if (pos_head.x - pos_tail.x).abs() > 1 || (pos_head.y - pos_tail.y).abs() > 1 {
                pos_tail.x += (pos_head.x - pos_tail.x).signum();
                pos_tail.y += (pos_head.y - pos_tail.y).signum();
            }

            visited.push(pos_tail.clone());
        }
    }

    visited.sort();
    visited.dedup();
    visited.len()
}

fn step_rope(rope: &mut PosVisited) {
    // loop over rope
    for tail in 1..rope.len() {
        let head = tail - 1;
        if (rope[head].x - rope[tail].x).abs() > 1 || (rope[head].y - rope[tail].y).abs() > 1 {
            rope[tail].x += (rope[head].x - rope[tail].x).signum();
            rope[tail].y += (rope[head].y - rope[tail].y).signum();
        }
    }
}

fn calculate2(input: &mut RiddleData) -> usize {
    let mut visited: PosVisited = vec![];

    // rope[0] is the head and rope[9] is the tail
    let mut rope: PosVisited = vec![Pos { x: 0, y: 0 }; 10];

    // iterate over steps
    for c in input.iter() {
        // iterate over counts
        for _ in 0..c.count {
            match c.dir {
                Direction::Right => rope[0].x += 1,
                Direction::Left => rope[0].x += -1,
                Direction::Up => rope[0].y += 1,
                Direction::Down => rope[0].y -= 1,
                Direction::None => {}
            }

            // simulate Tail
            step_rope(&mut rope);

            visited.push(rope[rope.len() - 1].clone());
        }
    }

    visited.sort();
    visited.dedup();
    visited.len()
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
    let riddle_text = common::readfile("data/inputday09-testdata-b.txt".to_string());
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate2(&mut riddledata);
    assert_eq!(output, 36);
}
