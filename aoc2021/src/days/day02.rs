use super::common;

#[derive(Default, Debug)]
struct Stearing {
    mov: String,
    dist: usize,
}

type StearVec = Vec<Stearing>;

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
        calculate_aim(riddle_vector)
    }
}
fn make_vec_from_string(csv: String) -> StearVec {
    let mut output = StearVec::default();
    let field = common::csvreader(&csv);
    for line in field.iter() {
        let mut record = line.iter();
        let mov = record.next().unwrap().to_string();
        let distx = record.next().unwrap().to_string();
        let dist = common::match_to_usize(distx);
        output.push(Stearing { mov, dist });
    }
    output
}

fn calculate(input: StearVec) -> usize {
    let mut x = 0;
    let mut y = 0;
    for record in input.iter() {
        match record.mov.as_ref() {
            "forward" => x += record.dist,
            "up" => y -= record.dist,
            "down" => y += record.dist,
            _ => {}
        }
    }
    x * y
}

fn calculate_aim(input: StearVec) -> usize {
    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;
    for record in input.iter() {
        match record.mov.as_ref() {
            "forward" => {
                x += record.dist;
                y += record.dist * aim;
            }
            "up" => aim -= record.dist,
            "down" => aim += record.dist,
            _ => {}
        }
        // println!("aim = {} :: x = {} :: y = {}", aim, x,y);
    }
    x * y
}
#[test]
fn riddle1() {
    let riddle_text = common::readfile("data/inputday02-testdata.txt".to_string());
    let riddle_vector = make_vec_from_string(riddle_text);
    let output = calculate(riddle_vector);
    assert_eq!(output, 150);
}

#[test]
fn riddle2() {
    let riddle_text = common::readfile("data/inputday02-testdata.txt".to_string());
    let riddle_vector = make_vec_from_string(riddle_text);
    let output = calculate_aim(riddle_vector);
    assert_eq!(output, 900);
}
