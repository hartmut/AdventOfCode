use super::common;

#[derive(Debug, Clone, Copy)]
enum SeatSearch {
    Front, //F lower half
    Back, //B upper half
    Left, //L left
    Right, //R right
    Empty, //empty
}
type OneSeat = [SeatSearch; 10];
type SeatVec = Vec<OneSeat>;

pub fn solve_riddle1(riddlefile: String) -> usize {
    let riddle_text = common::readfile(riddlefile.to_string());
    iterate(&mut make_seatvec_from_string(riddle_text))
}

pub fn solve_riddle2(riddlefile: String) -> usize {
    let riddle_text = common::readfile(riddlefile.to_string());
    iterate2(&mut make_seatvec_from_string(riddle_text))
}

fn make_seatvec_from_string(riddle_string: String) -> SeatVec {
    let mut seatvec: SeatVec = vec![];
    for l in riddle_string.lines() {
        let mut seat = create_empty_seat();
        let mut i = 0;
        for c in l.chars() {
            match c {
                'F' => seat[i] = SeatSearch::Front,
                'B' => seat[i] = SeatSearch::Back,
                'L' => seat[i] = SeatSearch::Left,
                'R' => seat[i] = SeatSearch::Right,
                _ => {},
            };
            i += 1;
        }
        seatvec.push(seat);
    }
    seatvec
}

fn get_seatno(seat: OneSeat) -> usize {
    let mut rowhalf = 64;
    let mut colhalf = 4;
    let mut rowmin = 0;
    let mut rowmax = 127;
    let mut colmin = 0;
    let mut colmax = 7;
    for i in 0..7 {
        match seat[i] {
            SeatSearch::Front => rowmax -= rowhalf, // I don't think i need this line
            SeatSearch::Back => rowmin += rowhalf,
            _ => {},
        }
        rowhalf /= 2;
    }
    for i in 0..3 {
        match seat[7+i] {
            SeatSearch::Left => colmax -= colhalf, // I don't think i need this line
            SeatSearch::Right => colmin += colhalf,
            _ => {},
        }
        colhalf /= 2;
    }
    rowmin * 8 + colmin
}

fn iterate(seatvec: &mut SeatVec) -> usize {
    let mut higest_seat = 0;
    while seatvec.len() > 0 {
        let seatnum = get_seatno(seatvec.pop().unwrap());
        if seatnum > higest_seat {higest_seat = seatnum};
    }
    higest_seat
}

fn iterate2(seatvec: &mut SeatVec) -> usize {
    let mut all_seats = [false;1024];
    while seatvec.len() > 0 {
        let seatnum = get_seatno(seatvec.pop().unwrap());
        all_seats[seatnum] = true;
    }
    let mut myseat = 0;
    while !all_seats[myseat] {myseat += 1;};
    while all_seats[myseat] {myseat += 1;};
    myseat
}

fn create_empty_seat() -> OneSeat {
    [SeatSearch::Empty;10]
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
fn seat_calc_test1() {
    let mut seatvec = make_seatvec_from_string("BFFFBBFRRR".to_string());
    let seatno = get_seatno(seatvec.pop().unwrap());
    assert_eq!(seatno, 567);
}

#[test]
fn seat_calc_test2() {
    let mut seatvec = make_seatvec_from_string("FFFBBBFRRR".to_string());
    let seatno = get_seatno(seatvec.pop().unwrap());
    assert_eq!(seatno, 119);
}

#[test]
fn seat_calc_test3() {
    let mut seatvec = make_seatvec_from_string("BBFFBBFRLL".to_string());
    let seatno = get_seatno(seatvec.pop().unwrap());
    assert_eq!(seatno, 820);
}

#[test]
fn seat_calc_test4() {
    let mut seatvec = make_seatvec_from_string("FBFBBFFRLR".to_string());
    let seatno = get_seatno(seatvec.pop().unwrap());
    assert_eq!(seatno, 357);
}

}
