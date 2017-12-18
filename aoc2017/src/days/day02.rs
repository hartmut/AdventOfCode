use super::file;
use super::day01;

type Minmax = (i32, i32);

pub fn to_vec(input: String) -> day01::Capvec {

    // init
    let mut output: day01::Capvec = Vec::new();
    let split = input.split("\t");

    // and now the loop
    for s in split {
        let i = s.parse::<i32>().unwrap();
        output.push(i);
    }
    output
}

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
    println!("the fist checksum is {:?}", sum);

    // loop for riddle 2 day 2
    reader = file::newreader(input.to_string());
    sum = 0;
    loop {
        match file::readline(&mut reader) {
            None => break,
            Some(x) => line = x,
        }
        // let s_slice: &str = &line[..];
        let vec = to_vec(line);
        sum += even_divisor(vec);
    }
    println!("the second checksum is {:?}", sum);
    println!("#######################################################");
}

fn even_divisor(vec: day01::Capvec) -> i32 {

    // init
    let len = vec.len();
    let mut outer = 0;
    let mut inner = 1;
    let mut factorout = 0;
    let mut multiple = 0;
    let mut divisor = 0;

    // and now loop alot
    while outer < len {
        while inner < len {
            // find divisors
            let val1 = *vec.get(outer).unwrap() as i32;
            let val2 = *vec.get(inner).unwrap() as i32;
            if val1 > val2 {
                multiple = val1;
                divisor = val2;
            } else {
                multiple = val2;
                divisor = val1;
            }
            if multiple % divisor == 0 {
                factorout = multiple / divisor
            };
            // next step
            inner += 1;
        }
        // next step
        outer += 1;
        inner = outer + 1;
    }
    factorout
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
