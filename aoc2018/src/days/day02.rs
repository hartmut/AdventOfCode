use super::common;

#[derive(Debug)]
struct CountApperance {
    pub twotimes: bool,
    pub threetimes: bool,
}

pub fn solve_day02_riddle1(riddlefile: String) -> i64 {
    // init
    let mut two = 0;
    let mut three = 0;
    let mut reader = common::newreader(riddlefile.to_string());

    //count
    loop {
        let line = match common::readline(&mut reader) {
            None => break,
            Some(x) => x,
        };
        let count_result = count_letters(line);
        if count_result.twotimes {
            two = two + 1;
        };
        if count_result.threetimes {
            three = three + 1;
        };
    }
    two * three
}

pub fn solve_day02_riddle2(riddlefile: String) -> String {
    let riddle_text = common::readfile(riddlefile.to_string());
    find_diff1_in_string_matrix(riddle_text)
}

fn count_letters(line: String) -> CountApperance {
    //init
    static ASCII_LOWER: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let mut counterlist = [0; 26];

    // count the chars
    let char_vec: Vec<char> = line.chars().collect();
    for c in char_vec {
        let index = ASCII_LOWER.iter().position(|r| &c == r).unwrap();
        counterlist[index] = counterlist[index] + 1;
    }
    //find the repeating chars
    let mut result = CountApperance {
        twotimes: false,
        threetimes: false,
    };
    for i in 0..26 {
        if counterlist[i] == 2 {
            result.twotimes = true;
        };
        if counterlist[i] == 3 {
            result.threetimes = true;
        };
    }
    result
}

fn find_diff1_in_string_matrix(boxids: String) -> String {
    let mut box_vec_outer = common::make_stringvec_from_string(boxids);
    let mut output = "!!we found nothing!!".to_string();

    'outer: while box_vec_outer.len() > 0 {
        let teststring1 = box_vec_outer.pop().unwrap();
        let mut box_vec_inner = box_vec_outer.clone();

        while box_vec_inner.len() > 0 {
            let teststring2 = box_vec_inner.pop().unwrap();

            if differ_by_one(teststring1.to_string(), teststring2.to_string()) {
                let mut char_vec1: Vec<char> = teststring1.chars().collect();
                let mut char_vec2: Vec<char> = teststring2.chars().collect();
                output = "".to_string();
                while char_vec1.len() > 0 {
                    let c = char_vec1.pop().unwrap();
                    if c == char_vec2.pop().unwrap() {
                        output.push(c);
                    };
                }
                break 'outer;
            }
        }
    }
    output.chars().rev().collect::<String>()
}

fn differ_by_one(line1: String, line2: String) -> bool {
    let mut char_vec1: Vec<char> = line1.chars().collect();
    let mut char_vec2: Vec<char> = line2.chars().collect();
    let mut diff = 0;

    while char_vec1.len() > 0 {
        if char_vec1.pop() != char_vec2.pop() {
            diff = diff + 1;
        };
    }

    if diff == 1 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_line(result: CountApperance, expected: CountApperance) {
        assert_eq!(result.twotimes, expected.twotimes);
        assert_eq!(result.threetimes, expected.threetimes);
    }

    #[test]
    fn check_letter_count1() {
        let testline = String::from("abcdef");
        check_line(
            count_letters(testline),
            CountApperance {
                twotimes: false,
                threetimes: false,
            },
        )
    }

    #[test]
    fn check_letter_count2() {
        let testline = String::from("bababc");
        check_line(
            count_letters(testline),
            CountApperance {
                twotimes: true,
                threetimes: true,
            },
        )
    }

    #[test]
    fn check_letter_count3() {
        let testline = String::from("abbcde");
        check_line(
            count_letters(testline),
            CountApperance {
                twotimes: true,
                threetimes: false,
            },
        )
    }
    #[test]
    fn check_letter_count4() {
        let testline = String::from("abcccd");
        check_line(
            count_letters(testline),
            CountApperance {
                twotimes: false,
                threetimes: true,
            },
        )
    }
    #[test]
    fn check_letter_count5() {
        let testline = String::from("aabcdd");
        check_line(
            count_letters(testline),
            CountApperance {
                twotimes: true,
                threetimes: false,
            },
        )
    }
    #[test]
    fn check_letter_count6() {
        let testline = String::from("abcdee");
        check_line(
            count_letters(testline),
            CountApperance {
                twotimes: true,
                threetimes: false,
            },
        )
    }
    #[test]
    fn check_letter_count7() {
        let testline = String::from("ababab");
        check_line(
            count_letters(testline),
            CountApperance {
                twotimes: false,
                threetimes: true,
            },
        )
    }

    #[test]
    fn find_the_box() {
        let testdata = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"
            .to_string();
        assert_eq!(find_diff1_in_string_matrix(testdata), "fgij".to_string());
    }

    #[test]
    fn compare_false_lines() {
        let testline1 = "abcde".to_string();
        let testline2 = "axcye".to_string();
        assert_eq!(differ_by_one(testline1, testline2), false);
    }

    #[test]
    fn compare_correct_lines() {
        let testline1 = "fguij".to_string();
        let testline2 = "fghij".to_string();
        assert_eq!(differ_by_one(testline1, testline2), true);
    }

}
