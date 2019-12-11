use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/input_day_three.txt")?;
    part_one(&input.trim());
    part_two(&input.trim());
    Ok(())
}

fn part_one(_input: &str) {

}

fn part_two(_input: &str) {

}

#[cfg(test)]
mod tests {
    use super::*;

//    #[test]
    fn day3_1_test_cases() {
        // assert_eq!(process("1,0,0,0,99",0,0), 2); // 1 + 1 = 2
        // assert_eq!(process("2,3,0,3,99",3,0), 2); // 3 * 2 = 6
        // assert_eq!(process("2,4,4,5,99,0",4,4), 2); // 99 * 99 = 9801
        // assert_eq!(process("1,1,1,4,99,5,6,0,99",1,1), 30);
    }

//    #[test]
    fn day3_2_test_cases() {
        // assert_eq!(process_sum("1,0,0,0,99",0,0), (2, 0));
        // assert_eq!(process_sum("2,3,0,3,99",3,0), (2, 300)); // 3 * 2 = 6
        // assert_eq!(process_sum("2,4,4,5,99,0",4,4), (2, 404)); // 99 * 99 = 9801
        // assert_eq!(process_sum("1,1,1,4,99,5,6,0,99",1,1), (30, 101));
    }
}
