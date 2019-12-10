use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/input1_1.txt")?;
    part_one(&input);
    part_two(&input);
    Ok(())
}

fn part_one(input: &str) {
    let answer: i32 = input
        .lines()
        .map(|line| fuel(line.parse().expect("Failed to parse number")))
        .sum();
    println!("The answer for day one is {}", answer);
}

fn part_two(input: &str) {
    let answer: i32 = input
        .lines()
        .map(|line| fuel_recursive(line.parse().expect("Failed to parse number")))
        .sum();
    println!("The answer for day two is {}", answer);
}

fn fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn fuel_recursive(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + fuel_recursive(fuel)
    }
}

#[cfg(test)]
mod tests {
    use super::{fuel, fuel_recursive};

    #[test]
    fn day1_1_test_cases() {
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
    }

    #[test]
    fn day1_2_test_cases() {
        assert_eq!(fuel_recursive(14), 2);
        assert_eq!(fuel_recursive(1969), 966);
        assert_eq!(fuel_recursive(100756), 50346);
    }
}
