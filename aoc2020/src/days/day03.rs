use super::common;
type Biome = Vec<Vec<u8>>;

pub fn solve_day03_riddle1(riddlefile: String) -> usize {
    let riddle_text = common::readfile(riddlefile.to_string());
    let biome = make_biome_from_string(riddle_text);
    iterate(biome, 3 ,1)
}

pub fn solve_day03_riddle2(riddlefile: String) -> usize {
    iterate2(riddlefile)
}

fn make_biome_from_string(riddle_string: String) -> Biome {
    let mut result_vec: Biome = vec![vec![]];
    let mut split = riddle_string.chars();
    let mut line = 0;

    for c in split {
        match c {
            // newline => line += 1,
            '\n' => {line += 1; result_vec.push(vec![]);},
            '#' => result_vec[line].push(1),
            '.' => result_vec[line].push(0),
            _ => println!("{:?}", c),
        }
    }
    let _ = result_vec.pop();
    result_vec
}

fn iterate(biome: Biome, right: usize, down: usize) -> usize {
    let width = biome[0].len();
    let mut line = 0;
    let mut column = 0;
    let mut counter = 0;

    while line < biome.len() {
        counter += usize::from(biome[line][column]);
        line += down;
        column = (column + right) % width;
    }
    counter
}

fn iterate2a(riddlefile: String, right: usize, down: usize) -> usize {
    let riddle_text = common::readfile(riddlefile.to_string());
    let biome = make_biome_from_string(riddle_text);
    iterate(biome, right, down)
}

fn iterate2(riddlefile: String) -> usize {
    let mut counter = iterate2a(riddlefile.clone(), 1,1);
    counter *= iterate2a(riddlefile.clone(), 3,1);
    counter *= iterate2a(riddlefile.clone(), 5,1);
    counter *= iterate2a(riddlefile.clone(), 7,1);
    counter *= iterate2a(riddlefile, 1,2);
    counter
}


#[cfg(test)]
mod tests {
    use super::*;

    fn testdata() -> Biome {
        let riddle_text = common::readfile("data/inputday3-test.txt".to_string());
        make_biome_from_string(riddle_text)
    }

    #[test]
    fn count_trees_3_1() {
        let biome = testdata();
        assert_eq!(iterate(biome, 3, 1), 7);
    }

    #[test]
    fn count_trees_1_1() {
        let biome = testdata();
        assert_eq!(iterate(biome, 1, 1), 2);
    }

    #[test]
    fn count_trees_5_1() {
        let biome = testdata();
        assert_eq!(iterate(biome, 5, 1), 3);
    }

    #[test]
    fn count_trees_the_second_way() {
        let biome = testdata();
        assert_eq!(iterate2("data/inputday3-test.txt".to_string()), 336);
    }

}
