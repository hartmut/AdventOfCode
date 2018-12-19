use super::common;

const LEN: usize = 1000;
type Fabric = Vec<Vec<usize>>;

#[derive(Debug)]
struct FabricPiece {
    record: usize,
    startx: usize,
    starty: usize,
    wide: usize,
    tall: usize,
}
impl FabricPiece {
    fn deconstruct(line: String) -> FabricPiece {
        let mut split = line.split_whitespace();
        let mut piece = FabricPiece {
            record: 0,
            startx: 0,
            starty: 0,
            wide: 0,
            tall: 0,
        };
        // get the record
        piece.record = match split.next() {
            Some(x) => {
                let mut a = x.chars();
                a.next();
                let num = a.map(|c| c).collect::<String>();
                common::match_to_usize(num)
            }
            None => panic!("theres something wrong in this element {}", line),
        };
        // get the start parts of the record
        // throwaway the @
        let s = split.next();
        let mut startpos = split.next().unwrap().split(",");
        piece.startx = match startpos.next() {
            Some(x) => common::match_to_usize(x.to_string()),
            None => panic!("theres something wrong in this element {}", line),
        };

        piece.starty = match startpos.next().unwrap().split(":").next() {
            Some(x) => common::match_to_usize(x.to_string()),
            None => panic!("theres something wrong in this element {}", line),
        };

        let mut startpos = split.next().unwrap().split("x");
        piece.wide = match startpos.next() {
            Some(x) => common::match_to_usize(x.to_string()),
            None => panic!("theres something wrong in this element {}", line),
        };
        piece.tall = match startpos.next() {
            Some(x) => common::match_to_usize(x.to_string()),
            None => panic!("theres something wrong in this element {}", line),
        };

        piece
    }
}

pub fn solve_day03_riddle1(riddlefile: String) -> usize {
    let riddle_text = common::readfile(riddlefile.to_string());
    let mut fabric: Fabric = vec![vec![0; LEN]; LEN];
    fill_fabric(riddle_text, &mut fabric);

    let mut counter = 0;
    for x in 0..LEN {
        for y in 0..LEN {
            if fabric[x][y] > 1 {
                counter = counter + 1;
            };
        }
    }
    counter
}

fn fill_fabric(riddle: String, mut fabric: &mut Fabric) {
    let lines = riddle.split("\n");
    for l in lines {
        match l {
            "" => break,
            _ => add_onepiece_to_fabric(FabricPiece::deconstruct(l.to_string()), &mut fabric),
        }
    }
}

fn add_onepiece_to_fabric(piece: FabricPiece, fabric: &mut Fabric) {
    for x in piece.startx..(piece.startx + piece.wide) {
        for y in piece.starty..(piece.starty + piece.tall) {
            fabric[y][x] = fabric[y][x] + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn checkup(expected: FabricPiece, result: FabricPiece) {
        assert_eq!(expected.record, result.record);
        assert_eq!(expected.startx, result.startx);
        assert_eq!(expected.starty, result.starty);
        assert_eq!(expected.wide, result.wide);
        assert_eq!(expected.tall, result.tall);
    }

    #[test]
    fn get_record() {
        let line = "#123 @ 3,2: 5x4".to_string();
        let expected_fabricrecord = FabricPiece {
            record: 123,
            startx: 3,
            starty: 2,
            wide: 5,
            tall: 4,
        };
        let deconstruct = FabricPiece::deconstruct(line);
        checkup(expected_fabricrecord, deconstruct);
    }
    #[test]
    fn get_record2() {
        let line = "#1 @ 896,863: 29x19".to_string();
        let expected_fabricrecord = FabricPiece {
            record: 1,
            startx: 896,
            starty: 863,
            wide: 29,
            tall: 19,
        };
        let deconstruct = FabricPiece::deconstruct(line);
        checkup(expected_fabricrecord, deconstruct);
    }
    #[test]
    fn get_record3() {
        let line = "#2 @ 367,315: 15x26".to_string();
        let expected_fabricrecord = FabricPiece {
            record: 2,
            startx: 367,
            starty: 315,
            wide: 15,
            tall: 26,
        };
        let deconstruct = FabricPiece::deconstruct(line);
        checkup(expected_fabricrecord, deconstruct);
    }

    #[test]
    fn add_onepiece_to_fabric() {
        const LEN: usize = 11;
        let mut fabric: Fabric = vec![vec![0; LEN]; LEN];
        let line = "#123 @ 3,2: 5x4".to_string();
        let piece = FabricPiece::deconstruct(line);
        super::add_onepiece_to_fabric(piece, &mut fabric);
        let testvec = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        for y in 0..fabric.len() {
            println!("{:?}", fabric[y]);
        }
        println!("####################",);
        for y in 0..testvec.len() {
            println!("{:?}", testvec[y]);
        }
        for x in 0..LEN {
            for y in 0..LEN {
                assert_eq!(fabric[x][y], testvec[x][y]);
            }
        }
    }

    #[test]
    fn add_morepieces_to_fabric() {
        const LEN: usize = 11;
        let mut fabric: Fabric = vec![vec![0; LEN]; LEN];
        let line = "#1 @ 1,3: 4x4".to_string();
        let piece = FabricPiece::deconstruct(line);
        super::add_onepiece_to_fabric(piece, &mut fabric);
        let line = "#2 @ 3,1: 4x4".to_string();
        let piece = FabricPiece::deconstruct(line);
        super::add_onepiece_to_fabric(piece, &mut fabric);
        let line = "#3 @ 5,5: 2x2".to_string();
        let piece = FabricPiece::deconstruct(line);
        super::add_onepiece_to_fabric(piece, &mut fabric);
        let testvec = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
            [0, 1, 1, 2, 2, 1, 1, 0, 0, 0, 0],
            [0, 1, 1, 2, 2, 1, 1, 0, 0, 0, 0],
            [0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
            [0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        for y in 0..fabric.len() {
            println!("{:?}", fabric[y]);
        }
        println!("####################",);
        for y in 0..testvec.len() {
            println!("{:?}", testvec[y]);
        }
        for x in 0..LEN {
            for y in 0..LEN {
                assert_eq!(fabric[x][y], testvec[x][y]);
            }
        }
    }
}
