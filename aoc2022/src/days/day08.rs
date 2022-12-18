use crate::days::common::match_to_usize;

use super::common::{self};

pub struct Solve;
type Grid = Vec<Vec<usize>>;

#[derive(Debug)]
struct RiddleData {
    data: Grid,
    visible: Grid,
}

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
    let mut output = RiddleData {
        data: vec![],
        visible: vec![],
    };

    for l in riddle_string.lines() {
        let mut linevec = vec![];
        let mut visiblevec = vec![];
        for c in l.chars() {
            let v = match_to_usize(c.to_string());
            linevec.push(v);
            visiblevec.push(0);
        }
        output.data.push(linevec);
        output.visible.push(visiblevec);
    }

    output
}

fn calculate(input: &mut RiddleData) -> usize {
    let mut score = 0;

    // boarder are visible
    let xmax = input.data[0].len();
    let ymax = input.data.len();

    for x in 0..xmax {
        input.visible[0][x] = 1;
        input.visible[ymax - 1][x] = 1;
    }

    for y in 0..ymax {
        input.visible[y][0] = 1;
        input.visible[y][xmax - 1] = 1;
    }

    // and now we need the inner visibility
    // first check the rows
    for y in 1..ymax {
        // from the left
        let mut xval = input.data[y][0];

        for x in 1..xmax {
            if input.data[y][x] > xval {
                input.visible[y][x] = 1;
                xval = input.data[y][x];
            }
        }

        // and from the right
        xval = input.data[y][xmax - 1];
        for x in (1..xmax).rev() {
            if input.data[y][x] > xval {
                input.visible[y][x] = 1;
                xval = input.data[y][x];
            }
        }
    }

    // and now the columns
    for x in 1..xmax {
        // from the top
        let mut yval = input.data[0][x];
        for y in 1..(ymax) {
            if input.data[y][x] > yval {
                input.visible[y][x] = 1;
                yval = input.data[y][x];
            }
        }

        // and from the bottom
        yval = input.data[ymax - 1][x];
        for y in (1..ymax).rev() {
            if input.data[y][x] > yval {
                input.visible[y][x] = 1;
                yval = input.data[y][x];
            }
        }
    }

    for l in input.visible.iter() {
        score += l.iter().sum::<usize>();
    }

    score
}

fn trace(x: usize, y: usize, xdir: i8, ydir: i8, input: &mut RiddleData) -> usize {
    let xmax = input.data[0].len();
    let ymax = input.data.len();
    let mut localscore = 1;
    let height = input.data[y][x];
    let mut x = x;
    let mut y = y;
    if xdir > 0 {
        x += 1;
    }
    if xdir < 0 {
        x -= 1;
    }
    if ydir > 0 {
        y += 1;
    }
    if ydir < 0 {
        y -= 1;
    }

    // loop
    loop {
        if input.data[y][x] < height && x > 0 && y > 0 && x < xmax - 1 && y < ymax - 1 {
            localscore += 1;
            if xdir > 0 {
                x += 1;
            }
            if xdir < 0 {
                x -= 1;
            }
            if ydir > 0 {
                y += 1;
            }
            if ydir < 0 {
                y -= 1;
            }
        } else {
            break;
        }
    }
    localscore
}

fn calculate2(input: &mut RiddleData) -> usize {
    let mut score = 0;

    let xmax = input.data[0].len();
    let ymax = input.data.len();

    // iterate over the inner trees
    for y in 1..ymax - 1 {
        for x in 1..xmax - 1 {
            // look right
            let mut localscore = trace(x, y, 1, 0, input);
            // look left
            localscore *= trace(x, y, -1, 0, input);
            // look down
            localscore *= trace(x, y, 0, 1, input);
            // look up
            localscore *= trace(x, y, 0, -1, input);
            // check
            if localscore > score {score = localscore};
        }
    }

    score
}

#[test]
fn riddle1() {
    let riddle_text = common::readfile("data/inputday08-testdata.txt".to_string());
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate(&mut riddledata);
    assert_eq!(output, 21);
}

#[test]
fn riddle2() {
    let riddle_text = common::readfile("data/inputday08-testdata.txt".to_string());
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate2(&mut riddledata);
    assert_eq!(output, 8);
}
