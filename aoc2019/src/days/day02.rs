use super::common;
use std::collections::HashSet;
pub type OpcodeVec = Vec<usize>;

pub fn solve_day02_riddle1(riddlefile: String) -> usize {
    let riddle_text = common::readfile(riddlefile.to_string());
    let mut riddle_vector = make_vec_from_string(riddle_text);
    iterate(&mut riddle_vector)
}

// pub fn solve_day02_riddle2(riddlefile: String) -> usize {
//     let riddle_text = common::readfile(riddlefile.to_string());
//     let riddle_vector = make_vec_from_string(riddle_text);
//     let mut output = 0;
//     for x in 1..98 {
//         for y in 1..98 {
//             let mut riddle_clone = riddle_vector.clone();
//             riddle_clone[1] = x;
//             riddle_clone[2] = y;
//             iterate(&mut riddle_clone);
//             if riddle_clone[0] == 19690720 {
//                 output = (x * 100) + y;
//                 break;
//             }
//         }
//     }
//     output
// }

fn make_vec_from_string(riddle_string: String) -> OpcodeVec {
    let split = riddle_string.split(",");
    let mut result_vec: OpcodeVec = vec![];

    for s in split {
        let without_whitespace = match s.split_whitespace().next() {
            None => break,
            Some(x) => match x.parse::<usize>() {
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

fn iterate(programm: &mut OpcodeVec) -> usize {
    let mut pos = 0;
    while (programm[pos] != 99) & (pos < programm.len()) {
        let i = programm[pos + 1].clone();
        let j = programm[pos + 2].clone();
        let k = programm[pos + 3].clone();
        match programm[pos] {
            1 => {
                programm[k] = programm[i] + programm[j];
            }
            2 => {
                programm[k] = programm[i] * programm[j];
            }
            _ => panic!("ups"),
        }
        pos += 4;
        // println!("{:?},{:?}", pos, programm);
    }

    programm[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smal_prog1() {
        let mut testvec = vec![1, 0, 0, 0, 99];
        let resultvec = vec![2, 0, 0, 0, 99];
        iterate(&mut testvec);
        for x in 0..testvec.len() {
            assert_eq!(resultvec[x], testvec[x]);
        }
    }
    #[test]
    fn smal_prog2() {
        let mut testvec = vec![2, 3, 0, 3, 99];
        let resultvec = vec![2, 3, 0, 6, 99];
        iterate(&mut testvec);
        for x in 0..testvec.len() {
            assert_eq!(resultvec[x], testvec[x]);
        }
    }
    #[test]
    fn smal_prog3() {
        let mut testvec = vec![2, 4, 4, 5, 99, 0];
        let resultvec = vec![2, 4, 4, 5, 99, 9801];
        iterate(&mut testvec);
        for x in 0..testvec.len() {
            assert_eq!(resultvec[x], testvec[x]);
        }
    }
    #[test]
    fn smal_prog4() {
        let mut testvec = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let resultvec = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        iterate(&mut testvec);
        for x in 0..testvec.len() {
            assert_eq!(resultvec[x], testvec[x]);
        }
    }
}
