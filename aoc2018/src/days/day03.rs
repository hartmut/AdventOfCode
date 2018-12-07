use super::common;

#[derive(Debug)]
struct FabricPiece {
    record: u16,
    startx: u16,
    starty: u16,
    wide: u16,
    tall: u16,
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
                common::match_to_u16(num)
            }
            None => panic!("theres something wrong in this element {}", line),
        };
        // get the start parts of the record

        let s = split.next();
        let mut startpos = split.next().unwrap().split(",");
        piece.startx = match startpos.next() {
            Some(x) => common::match_to_u16(x.to_string()),
            None => panic!("theres something wrong in this element {}", line),
        };

        piece.starty = match startpos.next().unwrap().split(":").next() {
            Some(x) => common::match_to_u16(x.to_string()),
            None => panic!("theres something wrong in this element {}", line),
        };

        let mut startpos = split.next().unwrap().split("x");
        piece.wide = match startpos.next() {
            Some(x) => common::match_to_u16(x.to_string()),
            None => panic!("theres something wrong in this element {}", line),
        };
        piece.tall = match startpos.next() {
            Some(x) => common::match_to_u16(x.to_string()),
            None => panic!("theres something wrong in this element {}", line),
        };

        piece
    }
}

pub fn solve_day03_riddle1(riddlefile: String) -> String {
    let riddle_text = common::readfile(riddlefile.to_string());
    // find_diff1_in_string_matrix(riddle_text)
    "hups".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(expected_fabricrecord.record, deconstruct.record);
        assert_eq!(expected_fabricrecord.startx, deconstruct.startx);
        assert_eq!(expected_fabricrecord.starty, deconstruct.starty);
        assert_eq!(expected_fabricrecord.wide, deconstruct.wide);
        assert_eq!(expected_fabricrecord.tall, deconstruct.tall);
    }
}
