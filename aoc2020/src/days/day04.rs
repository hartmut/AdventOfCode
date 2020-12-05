use super::common;

#[derive(Debug)]
struct Passport {
    byr: u16,    //Birth Year
    iyr: u16,    // Issue Year
    eyr: u16,    //Expiration Year
    hgt: String, //Height
    hcl: String, //Hair Color
    ecl: String, //Eye Color
    pid: String, //Passport ID
    cid: u16,    //Country ID
}
type PassportVec = Vec<Passport>;

pub fn solve_day04_riddle1(riddlefile: String) -> usize {
    let riddle_text = common::readfile(riddlefile.to_string());
    iterate(&mut make_passportvec_from_string(riddle_text))
}

pub fn solve_day04_riddle2(riddlefile: String) -> usize {
    let riddle_text = common::readfile(riddlefile.to_string());
    iterate(&mut make_passportvec_from_string(riddle_text))
}

fn make_passportvec_from_string(riddle_string: String) -> PassportVec {
    let mut passportvec: PassportVec = vec![];
    let mut lines = riddle_string.lines();
    let mut passport = init_passport();

    for s in lines {
        match s {
            "" => {
                passportvec.push(passport);
                passport = init_passport();
            }
            _ => fill_passport(s, &mut passport),
        }
    }
    passportvec.push(passport);
    passportvec
}

fn fill_passport(line: &str, passport: &mut Passport) {
    let mut l: Vec<&str> = line.split(' ').collect();
    for p in l {
        let s = p.split(':').collect::<Vec<&str>>();
        let element: &str = s[0];
        let value: &str = s[1];
        match element {
            "byr" => passport.byr = value.parse::<u16>().unwrap(),
            "iyr" => passport.iyr = value.parse::<u16>().unwrap(),
            "eyr" => passport.eyr = value.parse::<u16>().unwrap(),
            "hgt" => passport.hgt = value.to_string(),
            "hcl" => passport.hcl = value.to_string(),
            "ecl" => passport.ecl = value.to_string(),
            "pid" => passport.pid = value.to_string(),
            "cid" => passport.cid = value.parse::<u16>().unwrap(),
            _ => {}
        }
    }
}

fn init_passport() -> Passport {
    Passport {
        byr: 0,
        iyr: 0,
        eyr: 0,
        hgt: String::from(""),
        hcl: String::from(""),
        ecl: String::from(""),
        pid: String::from(""),
        cid: 0,
    }
}

fn iterate(passportvec: &mut PassportVec) -> usize {
    let mut counter = 0;
    while passportvec.len() > 0 {
        let mut pok = 0;
        let passport = passportvec.pop().unwrap();
        if passport.byr > 0 {
            pok += 1
        };
        if passport.iyr > 0 {
            pok += 1
        };
        if passport.eyr > 0 {
            pok += 1
        };
        if passport.hgt != "" {
            pok += 1
        };
        if passport.hcl != "" {
            pok += 1
        };
        if passport.ecl != "" {
            pok += 1
        };
        if passport.pid != "" {
            pok += 1
        };
        if pok == 7 {
            counter += 1;
        };
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_passport_from_different_lines() {
        let testline1 = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd";
        let testline2 = "byr:1937 iyr:2017 cid:147 hgt:183cm";
        let mut passport = init_passport();
        fill_passport(testline1, &mut passport);
        fill_passport(testline2, &mut passport);
        assert_eq!(passport.ecl, "gry");
        assert_eq!(passport.pid, "860033327");
        assert_eq!(passport.eyr, 2020);
        assert_eq!(passport.hcl, "#fffffd");
        assert_eq!(passport.byr, 1937);
        assert_eq!(passport.iyr, 2017);
        assert_eq!(passport.cid, 147);
        assert_eq!(passport.hgt, "183cm");
    }

    #[test]
    fn check_testdata() {
        let riddle_text = common::readfile("data/inputday04-test.txt".to_string());
        let mut passportvec = make_passportvec_from_string(riddle_text);
        assert_eq!(iterate(&mut passportvec), 2);
    }
}
