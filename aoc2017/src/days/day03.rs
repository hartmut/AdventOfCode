use std::f64;

pub fn distance_output(input: i32) {
    let dis = shell(input);
    // latteral movement is missing
    println!("the distance for day3 question 1 is {:?}", dis);
}

pub fn shell(input: i32) -> i32 {
    let outershell = (input as f64).sqrt() as i32;
    outershell / 2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test23() {
        let dis = shell(23);
        assert_eq!(dis, 2);
    }

    #[test]
    fn test49() {
        let dis = shell(49);
        assert_eq!(dis, 3);
    }
}
