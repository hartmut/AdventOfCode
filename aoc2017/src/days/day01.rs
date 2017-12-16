pub type Capvec = Vec<i32>;

pub fn captcha(input: &str) {
    let worklist = to_vec(input);
    // riddle 1
    let captcha = calculate(worklist.to_vec());
    println!("the first captcha is: {:?}\n", captcha);
    // riddle 2
    let captcha = calculate_around(worklist);
    println!("and the second captcha is: {:?}\n", captcha);
}

pub fn to_vec(input: &str) -> Capvec {
    let mut output: Capvec = Vec::new();
    let mut input = input.to_string();
    while input.len() > 0 {
        let c = input.remove(0).to_string();
        let i = c.parse::<i32>().unwrap();
        output.push(i);
    }
    output
}

fn calculate(mut input: Capvec) -> i32 {
    let mut sum: i32 = 0;
    let mut a = input.pop().unwrap();
    let end = a;
    while input.len() > 0 {
        let b = input.pop().unwrap();
        if b == a {
            sum += a;
        }
        a = b;
    }
    if end == a {
        sum += a
    };
    sum
}

fn calculate_around(input: Capvec) -> i32 {
    let len = input.len();
    let mut counter = 0;
    let mut plushalf = len / 2;
    let mut sum = 0;
    while counter < len {
        if input.get(counter).unwrap() == input.get(plushalf).unwrap() {
            sum += *input.get(counter).unwrap() as i32;
        }
        plushalf += 1;
        if plushalf == len {
            plushalf = 0
        };
        counter += 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_halfway_around4() {
        let vector = to_vec("12131415");
        let sum = calculate_around(vector);
        assert_eq!(sum, 4);
    }

    #[test]
    fn calculate_halfway_around12() {
        let vector = to_vec("123123");
        let sum = calculate_around(vector);
        assert_eq!(sum, 12);
    }

    #[test]
    fn create_vec() {
        let vector = to_vec("91212129");
        assert_eq!(vector, [9, 1, 2, 1, 2, 1, 2, 9]);
    }

    #[test]
    fn cacluate_the_captcha_case9() {
        let vector = to_vec("91212129");
        let sum = calculate(vector);
        assert_eq!(sum, 9);
    }

    #[test]
    fn cacluate_the_captcha_case4() {
        let vector = to_vec("1111");
        let sum = calculate(vector);
        assert_eq!(sum, 4);
    }

    #[test]
    fn cacluate_the_captcha_case12() {
        let vector = to_vec("9121221129");
        let sum = calculate(vector);
        assert_eq!(sum, 12);
    }
}
