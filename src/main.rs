#![allow(dead_code)]

use std::error::Error;

mod day_one;
mod day_two;

fn main() -> Result<(), Box<dyn Error>> {
    day_two::run()
}
