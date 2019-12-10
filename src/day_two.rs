use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/input_day_two.txt")?;
    part_one(&input.trim());
    part_two(&input);
    Ok(())
}

fn part_one(input: &str) {
    println!("The answer for part one is {}", process(&input));
}

fn part_two(_input: &str) {
    // let answer: i32 = input
    //     .lines()
    //     .map(|line| fuel_recursive(line.parse().expect("Failed to parse number")))
    //     .sum();
    // println!("The answer for day two is {}", answer);
}

fn process(input: &str) -> i32 {
    let mut registers: Vec<i32> = input.split(',').map(|register| {
        println!("{}", register);
        return register.parse().expect("Failed to parse number")
    }).collect();

    registers[1] = 12;
    registers[2] = 1;

    //let mut result: String = String::new();
    // Do the actual processing here
    let mut index = 0;
    loop {
        match registers[index] {
            1 => { 
                // Add
                let op1_index = registers[index + 1] as usize;
                let op2_index = registers[index + 2] as usize;
                let sum_index = registers[index + 3] as usize;
                registers[sum_index] = registers[op1_index] + registers[op2_index];
            },
            2 => {
                // Multiply
                let op1_index = registers[index + 1] as usize;
                let op2_index = registers[index + 2] as usize;
                let sum_index = registers[index + 3] as usize;
                registers[sum_index] = registers[op1_index] * registers[op2_index];
            },
            99 => { break;},
            _ => {},
        }
        index += 4;
        if index >= registers.len() { break; }
    }
    
    registers[0]
}

#[cfg(test)]
mod tests {
    use super::{process};

    #[test]
    fn day2_1_test_cases() {
        assert_eq!(process("1,0,0,0,99"), 2); // 1 + 1 = 2
        assert_eq!(process("2,3,0,3,99"), 2); // 3 * 2 = 6
        assert_eq!(process("2,4,4,5,99,0"), 2); // 99 * 99 = 9801
        assert_eq!(process("1,1,1,4,99,5,6,0,99"), 30);
    }

    // #[test]
    // fn day2_2_test_cases() {

    // }
}
