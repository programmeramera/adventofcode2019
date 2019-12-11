#![allow(dead_code)]

use std::error::Error;

mod day_one;
mod day_two;
mod day_three;

fn main() -> Result<(), Box<dyn Error>> {
    day_three::run()
}
