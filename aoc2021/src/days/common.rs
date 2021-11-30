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

pub fn readfile(riddlefile: String) -> String {
    let mut f = newreader(riddlefile.to_string());
    let mut file_content = String::new();
    let i = f.read_to_string(&mut file_content);
    if let Err(error) = i {
        println!("{:?}", error)
    }
    file_content
}

pub fn print_separator() {
    println!("##############################################################################################################");
}

pub fn print_day_and_riddle(day: u32, riddle: u32) {
    println!("we are on day {} and this is riddle {}", day, riddle);
}
