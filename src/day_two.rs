use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/input_day_two.txt")?;
    part_one(&input.trim());
    part_two(&input.trim());
    Ok(())
}

fn part_one(input: &str) {
    println!("The answer for day two, part one, is {}", process(&input, 12, 2));
}

fn part_two(input: &str) {
    for noun in 0..100 {
        for verb in 0..100 {
            let (result, sum) = process_sum(&input.clone(), noun, verb);
            if result == 19690720 { 
                println!("The answer for day two, part two, is: {}", sum);
                return;
            }
        }
    }
}

fn process_sum(input: &str, noun: i32, verb: i32) -> (i32, i32) {
    (process(&input.clone(), noun, verb), 100 * noun + verb)
}

fn process(input: &str, noun: i32, verb: i32) -> i32 {
    let mut registers: Vec<i32> = input.split(',').map(|register| {
        return register.parse().expect("Failed to parse number")
    }).collect();

    registers[1] = noun;
    registers[2] = verb;

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
    use super::{process, process_sum};

    #[test]
    fn day2_1_test_cases() {
        assert_eq!(process("1,0,0,0,99",0,0), 2); // 1 + 1 = 2
        assert_eq!(process("2,3,0,3,99",3,0), 2); // 3 * 2 = 6
        assert_eq!(process("2,4,4,5,99,0",4,4), 2); // 99 * 99 = 9801
        assert_eq!(process("1,1,1,4,99,5,6,0,99",1,1), 30);
    }

    #[test]
    fn day2_2_test_cases() {
        assert_eq!(process_sum("1,0,0,0,99",0,0), (2, 0));
        assert_eq!(process_sum("2,3,0,3,99",3,0), (2, 300)); // 3 * 2 = 6
        assert_eq!(process_sum("2,4,4,5,99,0",4,4), (2, 404)); // 99 * 99 = 9801
        assert_eq!(process_sum("1,1,1,4,99,5,6,0,99",1,1), (30, 101));
    }
}
