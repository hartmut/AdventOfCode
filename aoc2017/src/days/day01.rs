type capvec = Vec<i32>;

pub fn captcha(input: &str) {
    let mut worklist = to_vec(input);
    let captcha = calculate(worklist);
    println!("and the first captcha is: {:?}\n", captcha);
}

fn to_vec(input: &str) -> capvec {
    let mut output: capvec = Vec::new();
    let mut input = input.to_string();
    while input.len() > 0 {
        let c = input.remove(0).to_string();
        let i = c.parse::<i32>().unwrap();
        output.push(i);
    }
    output
}

fn calculate(mut input: capvec) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_vec() {
        let vector = to_vec("91212129");
        assert_eq!(vector, [9, 1, 2, 1, 2, 1, 2, 9]);
    }

    #[test]
    fn cacluate_the_captcha_case1() {
        let vector = to_vec("91212129");
        let sum = calculate(vector);
        assert_eq!(sum, 9);
    }

    #[test]
    fn cacluate_the_captcha_case2() {
        let vector = to_vec("1111");
        let sum = calculate(vector);
        assert_eq!(sum, 4);
    }

    #[test]
    fn cacluate_the_captcha_case3() {
        let vector = to_vec("9121221129");
        let sum = calculate(vector);
        assert_eq!(sum, 12);
    }
}
