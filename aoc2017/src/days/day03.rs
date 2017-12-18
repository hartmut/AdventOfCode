pub fn distance_output(input: i32) {
    // calculate
    let output = calculate(input);
    // output question 1 - currently not working
    println!("the first distance on day3 is {:?}", output);

    println!("#######################################################");
}

fn calculate(input: i32) -> i32 {
    let depth = shell_depth(input);
    let steps = input - (2 * depth - 1).pow(2);
    let se_rest = steps % (depth * 2);
    let se_path = (se_rest - depth).abs();
    se_path + depth
}

fn shell_depth(input: i32) -> i32 {
    ((((input - 1) as f64).sqrt() as i32) + 1) / 2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_depth() {
        assert_eq!(shell_depth(23), 2);
        assert_eq!(shell_depth(49), 3);
        assert_eq!(shell_depth(50), 4);
        assert_eq!(shell_depth(1024), 16);
    }

    #[test]
    fn test_calculation() {
        assert_eq!(calculate(23), 2);
        assert_eq!(calculate(22), 3);
        assert_eq!(calculate(25), 4);
        assert_eq!(calculate(12), 3);
        assert_eq!(calculate(1024), 31);
    }

}
