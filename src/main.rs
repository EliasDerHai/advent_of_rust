use std::env;
use crate::one::{solve_day_01_part_1, solve_day_01_part_2};
use crate::two::{solve_day_02_part_1, solve_day_02_part_2};
use crate::three::{solve_day_03_part_1, solve_day_03_part_2};
use crate::four::{solve_day_04_part_1, solve_day_04_part_2};
use crate::five::{solve_day_05_part_1, solve_day_05_part_2};

mod util;
mod one;
mod two;
mod three;
mod four;
mod five;

fn main() {
    let args: Vec<String> = env::args().collect();
    let solve_arg = args.get(1);
    match solve_arg {
        None => println!("Missing argument for kata to solve"),
        Some(value) => {
            match solve_day_and_part(value.as_str()) {
                Ok(value) => { println!("Final answer: {value}"); }
                Err(err) => { println!("{err}") }
            }
        }
    }
}

fn solve_day_and_part(arg: &str) -> Result<u32, &str> {
    match arg {
        "11" => Ok(solve_day_01_part_1()),
        "12" => Ok(solve_day_01_part_2()),
        "21" => Ok(solve_day_02_part_1()),
        "22" => Ok(solve_day_02_part_2()),
        "31" => Ok(solve_day_03_part_1()),
        "32" => Ok(solve_day_03_part_2()),
        "41" => Ok(solve_day_04_part_1()),
        "42" => Ok(solve_day_04_part_2()),
        "51" => Ok(solve_day_05_part_1()),
        "52" => Ok(solve_day_05_part_2()),
        _ => Err("Not yet implemented for ")
    }
}

