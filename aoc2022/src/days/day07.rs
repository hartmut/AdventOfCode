use crate::days::common::match_to_usize;

use super::common::{self};
use std::collections::HashMap;

pub struct Solve;

type RiddleData = String;

type FileSystem = HashMap<String, usize>;

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

fn massage_data(input: &mut RiddleData) -> FileSystem {
    let mut filesystem: FileSystem = HashMap::new();
    let mut path = "".to_string();

    // build filesystemstructure
    for l in input.lines() {
        if l.starts_with("$ cd ") {
            let act = l.get(5..).unwrap();
            if act.starts_with("..") {
                let (left, _) = path.rsplit_once("/").unwrap();
                path = left.to_string();
                if path.len() == 0 {
                    path = "/".to_string()
                };
            } else {
                if path.len() <= 1 {
                    path = path + act;
                } else {
                    path = path + "/" + act;
                }
            }
        } else {
            if l.starts_with("$ ls") || l.starts_with("dir") {
            } else {
                let (size, name) = l.rsplit_once(" ").unwrap();
                let size = match_to_usize(size.to_string());
                let mut filename = path.clone();
                if path.len() == 1 {
                    filename = filename + name;
                } else {
                    filename = filename + "/" + name;
                }
                filesystem.insert(filename, size);
            }
        }
    }

    // calculate sums of directories
    let mut directories: FileSystem = HashMap::new();
    for (file, size) in filesystem {
        // iterate over subdirectories
        let (mut left, _) = file.rsplit_once('/').unwrap();
        while left.len() > 0 {
            let sum = directories.get(left);
            match sum {
                Some(x) => {
                    let update = x + size;
                    directories.insert(left.to_string(), update);
                }
                None => {
                    directories.insert(left.to_string(), size);
                }
            }
            (left, _) = left.rsplit_once('/').unwrap();
        }
        let sum = directories.get("/");
        match sum {
            Some(x) => {
                let update = x + size;
                directories.insert("/".to_string(), update);
            }
            None => {
                directories.insert("/".to_string(), size);
            }
        }
    }
    directories
}

fn calculate(input: &mut RiddleData) -> usize {
    // create datastructures
    let directories = massage_data(input);

    // calculate sum of directories <= 100000
    let mut score = 0;
    for (_, size) in directories {
        // iterate over subdirectories
        if size <= 100000 {
            score += size;
        }
    }
    score
}

fn calculate2(input: &mut RiddleData) -> usize {
    // create datastructures
    let directories = massage_data(input);

    // calculate total of filesystem
    let total = directories.get("/").unwrap().clone();

    // find the smalest directory to delete and answer with size
    let tofree = total - 40000000;
    let mut score = total;
    
    for (_, size) in directories {
        // iterate over subdirectories
        if size >= tofree && size < score {
            score = size;
        }
    }
    score
}

#[test]
fn riddle1() {
    let riddle_text = common::readfile("data/inputday07-testdata.txt".to_string());
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate(&mut riddledata);
    assert_eq!(output, 95437);
}

#[test]
fn riddle2() {
    let riddle_text = common::readfile("data/inputday07-testdata.txt".to_string());
    let mut riddledata = make_riddle_data(riddle_text);
    let output = calculate2(&mut riddledata);
    assert_eq!(output, 24933642);
}
