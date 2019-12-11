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

fn manhattan_distance(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_1_test_cases() {
        assert_eq!(
            manhattan_distance(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
                U62,R66,U55,R34,D71,R55,D58,R83")
            , 159);
        assert_eq!(
            manhattan_distance(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
                U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
            , 135);
    }

//    #[test]
    fn day3_2_test_cases() {
        // assert_eq!(process_sum("1,0,0,0,99",0,0), (2, 0));
        // assert_eq!(process_sum("2,3,0,3,99",3,0), (2, 300)); // 3 * 2 = 6
        // assert_eq!(process_sum("2,4,4,5,99,0",4,4), (2, 404)); // 99 * 99 = 9801
        // assert_eq!(process_sum("1,1,1,4,99,5,6,0,99",1,1), (30, 101));
    }
}
