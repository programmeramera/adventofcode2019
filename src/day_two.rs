use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/input_day_one.txt")?;
    part_one(&input);
    part_two(&input);
    Ok(())
}

fn part_one(_input: &str) {
    // let answer: i32 = input
    //     .lines()
    //     .map(|line| fuel(line.parse().expect("Failed to parse number")))
    //     .sum();
    // println!("The answer for day one is {}", answer);
}

fn part_two(_input: &str) {
    // let answer: i32 = input
    //     .lines()
    //     .map(|line| fuel_recursive(line.parse().expect("Failed to parse number")))
    //     .sum();
    // println!("The answer for day two is {}", answer);
}

fn process(input: &str) -> &str {
    input
}

#[cfg(test)]
mod tests {
    use super::{process};

    #[test]
    fn day2_1_test_cases() {
        assert_eq!(process("1,0,0,0,99"), "1,0,0,0,99");
        assert_eq!(process("2,3,0,3,99"), "2,3,0,6,99");
        assert_eq!(process("2,4,4,5,99,0"), "2,4,4,5,99,9801");
        assert_eq!(process("1,1,1,4,99,5,6,0,99"), "30,1,1,4,2,5,6,0,99");
    }

    #[test]
    fn day2_2_test_cases() {

    }
}
