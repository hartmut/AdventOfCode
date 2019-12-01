use super::common;
use std::collections::HashSet;
pub type MassVec = Vec<i64>;

pub fn solve_day01_riddle1(riddlefile: String) -> i64 {
    let riddle_text = common::readfile(riddlefile.to_string());
    let riddle_vector = make_vec_from_string(riddle_text);
    calculate(riddle_vector)
}

pub fn solve_day01_riddle2(riddlefile: String) -> i64 {
    let riddle_text = common::readfile(riddlefile.to_string());
    let riddle_vector = make_vec_from_string(riddle_text);
    all_fuel_needed(riddle_vector)
}

fn make_vec_from_string(riddle_string: String) -> MassVec {
    let split = riddle_string.split("\n");
    let mut result_vec: MassVec = vec![];

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

fn calculate(input: MassVec) -> i64 {
    let mut fuel: i64 = 0;
    for x in input {
        fuel = fuel + mass_to_fuel(x);
    }
    fuel
}

fn all_fuel_needed(input: MassVec) -> i64 {
    let mut fuel: i64 = 0;
    for x in input {
        let ModuleFuel = mass_to_fuel(x);
        fuel = fuel + recurse(ModuleFuel);
    }
    fuel
}

fn recurse(input: i64) -> i64 {
    let mut fuel = input;
    let mut result = input;
    while result > 0 {
        result = mass_to_fuel(result);
        if result > 0 {
            fuel = fuel + result;
        }
    }
    fuel
}

fn mass_to_fuel(mass: i64) -> i64 {
    (mass / 3) - 2
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
    fn fuel_calc_per_module() {
        let mut testvec = vec![12, 14, 1969, 100756];
        let mut resultvec = vec![2, 2, 654, 33583];
        for i in 1..testvec.len() {
            let fuel = mass_to_fuel(testvec.pop().unwrap());
            assert_eq!(fuel, resultvec.pop().unwrap());
        }
    }
    #[test]
    fn fuel_calc_for_all() {
        let mut testvec = vec![14, 1969, 100756];
        let mut resultvec = vec![2, 966, 50346];
        for i in 1..testvec.len() {
            let fuel = recurse(mass_to_fuel(testvec.pop().unwrap()));
            assert_eq!(fuel, resultvec.pop().unwrap());
        }
    }
}
