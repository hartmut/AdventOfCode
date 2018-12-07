use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

type StringVec = Vec<String>;

pub fn newreader(filename: String) -> BufReader<File> {
    let path = Path::new(&filename);
    let f = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };
    BufReader::new(f)
}

pub fn readline(f: &mut BufReader<File>) -> Option<String> {
    let mut line = String::new();
    if f.read_line(&mut line).unwrap() == 0 {
        None
    } else {
        let lastchar = line.pop().unwrap();

        if lastchar != '\n' {
            line.push(lastchar)
        }

        Some(line)
    }
}

pub fn readfile(riddlefile: String) -> String {
    let mut f = newreader(riddlefile.to_string());
    let mut file_content = String::new();
    let i = f.read_to_string(&mut file_content);
    match i {
        Err(error) => println!("{:?}", error),
        Ok(_) => (),
    }
    file_content
}

pub fn make_stringvec_from_string(riddle_string: String) -> StringVec {
    let split = riddle_string.split("\n");
    let mut result_vec: StringVec = vec![];

    for s in split {
        let without_whitespace = match s.split_whitespace().next() {
            None => break,
            Some(x) => result_vec.push(x.to_string()),
        };
    }

    result_vec
}

pub fn print_separator() {
    println!("##############################################################################################################");
}

pub fn match_to_u16(num: String) -> u16 {
    match num.parse::<u16>() {
        Ok(x) => x,
        Err(x) => {
            panic!("not a number {:?}", x);
        }
    }
}
