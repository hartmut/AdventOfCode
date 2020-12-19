use super::common;

type GroupVec = Vec<String>;

pub fn solve_riddle1(riddlefile: String) -> usize {
    let riddle_text = common::readfile(riddlefile);
    iterate(&mut make_groupvec_from_string(riddle_text))
}

pub fn solve_riddle2(riddlefile: String) -> usize {
    let riddle_text = common::readfile(riddlefile);
    iterate2(&mut make_groupvec_from_string2(riddle_text))
}

fn make_groupvec_from_string(riddle_string: String) -> GroupVec {
    let mut group_vec: GroupVec = vec![];
    let mut group = "".to_string();
    for l in riddle_string.lines() {
        match l {
            "" => {
                group_vec.push(group);
                group = "".to_string();
            }
            _ => {
                for c in l.chars() {
                    if !group.contains(c) {
                        group.push(c);
                    }
                }
            }
        }
    }
    group_vec.push(group);
    group_vec
}

fn make_groupvec_from_string2(riddle_string: String) -> GroupVec {
    let mut group_vec: GroupVec = vec![];
    for l in riddle_string.lines() {
        group_vec.push(l.to_string());
    }
    group_vec
}

fn iterate(group_vec: &mut GroupVec) -> usize {
    let mut sum = 0;
    while group_vec.len() > 0 {
        sum += group_vec.pop().unwrap().len();
    }
    sum
}

fn iterate2(group_vec: &mut GroupVec) -> usize {
    let mut sum = 0;
    let mut check: String = group_vec.pop().unwrap();

    while group_vec.len() > 0 {
        let line = group_vec.pop().unwrap();
        match line.len() {
            0 => {
                sum += check.len();
                if group_vec.len() > 0 {
                    check = group_vec.pop().unwrap();
                };
            }
            _ => {
                let mut check2 = "".to_string();
                for c in line.chars() {
                    if check.contains(c) {
                        check2.push(c)
                    }
                }
                check = check2;
            }
        }
    }
    sum += check.len();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_testanswers() {
        let riddle_text = common::readfile("data/inputday06-test.txt".to_string());
        assert_eq!(iterate(&mut make_groupvec_from_string(riddle_text)), 11);
    }

    #[test]
    fn sum_of_all_yes() {
        let riddle_text = common::readfile("data/inputday06-test.txt".to_string());
        assert_eq!(iterate2(&mut make_groupvec_from_string2(riddle_text)), 6);
    }
}
