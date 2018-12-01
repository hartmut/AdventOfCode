use super::common;
use std::collections::HashSet;
pub type Freqvec = Vec<i64>;

pub fn solve_day01_riddle1(riddlefile: String) -> i64 {
    let riddle_text = common::readfile(riddlefile.to_string());
    let riddle_vector = make_vec_from_string(riddle_text);
    calculate(riddle_vector)
}

pub fn solve_day01_riddle2(riddlefile: String) -> i64 {
    let riddle_text = common::readfile(riddlefile.to_string());
    let riddle_vector = make_vec_from_string(riddle_text);
    search_double(riddle_vector).unwrap()
}

fn make_vec_from_string(riddle_string: String) -> Freqvec {
    let split = riddle_string.split("\n");
    let mut result_vec: Freqvec = vec![];

    for s in split {
        let without_whitespace = match s.split_whitespace().next() {
            None => break,
            Some(x) => match x.parse::<i64>() {
                Err(x) => {
                    println!("{:?}", x);
                    break;
                }
                Ok(x) => x,
            },
        };
        result_vec.push(without_whitespace);
    }

    result_vec
}

fn search_double(input: Freqvec) -> Option<i64> {
    let mut frequency: i64 = 0;
    let mut found: Option<i64> = None;
    let mut counter = 0;
    let mut allready_seen_freq = HashSet::new();
    allready_seen_freq.insert(0);

    loop {
        frequency = frequency + input[counter];
        if allready_seen_freq.contains(&frequency) {
            found = Some(frequency);
            break;
        };
        allready_seen_freq.insert(frequency);
        counter = counter + 1;
        if counter >= input.len() {
            counter = 0;
        }
    }
    found
}

fn calculate(input: Freqvec) -> i64 {
    //starts at 0 in the examples and for the real case
    let mut frequency: i64 = 0;
    for x in input {
        frequency = frequency + x;
    }
    frequency
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn make_vec() {
        let testdata = String::from(
            "-12
    -6
    -12
    +1",
        );
        assert_eq!(make_vec_from_string(testdata), vec![-12, -6, -12, 1])
    }
    #[test]
    fn test_cases_rid1_1() {
        let example1: Freqvec = vec![1, 1, 1];
        assert_eq!(calculate(example1), 3);
    }

    #[test]
    fn test_cases_rid1_2() {
        let example2: Freqvec = vec![1, 1, -2];
        assert_eq!(calculate(example2), 0);
    }

    #[test]
    fn test_cases_rid1_3() {
        let example3: Freqvec = vec![-1, -2, -3];
        assert_eq!(calculate(example3), -6);
    }

    #[test]
    fn test_case_rid2_0() {
        let example: Freqvec = vec![1, -2, 3, 1, 1, -2];
        assert_eq!(search_double(example), Some(2));
    }

    #[test]
    fn test_case_rid2_1() {
        let example: Freqvec = vec![1, -1];
        assert_eq!(search_double(example), Some(0));
    }

    #[test]
    fn test_case_rid2_2() {
        let example: Freqvec = vec![3, 3, 4, -2, -4];
        assert_eq!(search_double(example), Some(10));
    }

    #[test]
    fn test_case_rid2_3() {
        let example: Freqvec = vec![-6, 3, 8, 5, -6];
        let result = 5;
        assert_eq!(search_double(example), Some(5));
    }

    #[test]
    fn test_case_rid2_4() {
        let example: Freqvec = vec![7, 7, -2, -7, -4];
        assert_eq!(search_double(example), Some(14));
    }

}
