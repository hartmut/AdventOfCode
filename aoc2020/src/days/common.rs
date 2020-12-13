use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;


pub fn newreader(filename: String) -> BufReader<File> {
    let path = Path::new(&filename);
    let f = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", path.display()),
        Ok(file) => file,
    };
    BufReader::new(f)
}

// pub fn readline(f: &mut BufReader<File>) -> Option<String> {
//     let mut line = String::new();
//     if f.read_line(&mut line).unwrap() == 0 {
//         None
//     } else {
//         let lastchar = line.pop().unwrap();
//
//         if lastchar != '\n' {
//             line.push(lastchar)
//         }
//
//         Some(line)
//     }
// }

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

pub fn print_separator() {
    println!("##############################################################################################################");
}

pub fn print_day_and_riddle(day: u32, riddle: u32) {
    println!("we are on day {} and this is riddle {}", day, riddle);
}

// pub fn match_to_u16(num: String) -> u16 {
//     match num.parse::<u16>() {
//         Ok(x) => x,
//         Err(x) => {
//             panic!("not a number {:?}", x);
//         }
//     }
// }
//
// pub fn match_to_usize(num: String) -> usize {
//     match num.parse::<usize>() {
//         Ok(x) => x,
//         Err(x) => {
//             panic!("not a number {:?}", x);
//         }
//     }
// }
