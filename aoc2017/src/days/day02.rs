use super::file;
use super::day01;

type Minmax = (i32, i32);

pub fn checksum(input: String) {

    // init
    let mut reader = file::newreader(input.to_string());
    let mut line = String::new();
    let mut sum: i32 = 0;

    // loop for riddle 1 day 2
    loop {
        match file::readline(&mut reader) {
            None => break,
            Some(x) => line = x,
        }
        let (min, max) = get_min_max(line);
        sum = sum + (max - min);
    }
    println!("the checksum for the second day riddle 1 is {:?}\n", sum);

    // loop for riddle 2 day 2
    reader = file::newreader(input.to_string());
    sum = 0;
    loop {
        match file::readline(&mut reader) {
            None => break,
            Some(x) => line = x,
        }
        let s_slice: &str = &line[..];
        let vec = day01::to_vec(s_slice);
        sum += even_divisor(vec);
    }
    println!("the checksum for the second day riddle 2 is {:?}\n", sum);
}

fn even_divisor(vec: day01::Capvec) -> i32 {

    // init
    let len = vec.len();
    let mut outer = 0;
    let mut inner = 1;

    // and now loop alot
    while outer < len {
        while inner < len {

            inner += 1;
        }
        outer += 1;
    }

    4
}

fn get_min_max(input: String) -> Minmax {

    // init
    let mut min = i32::max_value();
    let mut max = 0;
    let split = input.split("\t");

    // and now the loop
    for s in split {
        let i = s.parse::<i32>().unwrap();
        if i < min {
            min = i
        };
        if i > max {
            max = i
        };
    }
    // and last but not leat send the result back
    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice_text_test() {
        let example = "5\t1\t9\t5".to_string();
        let output = get_min_max(example);
        assert_eq!(output, (1, 9));
    }

    #[test]
    fn even_divisor_line1() {
        let example: day01::Capvec = vec![5, 9, 2, 8];
        let output = even_divisor(example);
        assert_eq!(output, 4);
    }

    #[test]
    fn even_divisor_line2() {
        let example: day01::Capvec = vec![9, 4, 7, 3];
        let output = even_divisor(example);
        assert_eq!(output, 3);
    }

    #[test]
    fn even_divisor_line3() {
        let example: day01::Capvec = vec![3, 8, 6, 5];
        let output = even_divisor(example);
        assert_eq!(output, 2);
    }

}
