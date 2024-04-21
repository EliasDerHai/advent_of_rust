use std::env;
use crate::one::{solve_day_01_part_1, solve_day_01_part_2};
use crate::two::{solve_day_02_part_1, solve_day_02_part_2};
use crate::three::{solve_day_03_part_1, solve_day_03_part_2};
use crate::four::{solve_day_04_part_1, solve_day_04_part_2};

mod util;
mod one;
mod two;
mod three;
mod four;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    match args.get(1) {
        None => 0,
        Some(arg) => {
            match arg.as_str() {
                "11" => solve_day_01_part_1(),
                "12" => solve_day_01_part_2(),
                "21" => solve_day_02_part_1(),
                "22" => solve_day_02_part_2(),
                "31" => solve_day_03_part_1(),
                "32" => solve_day_03_part_2(),
                "41" => solve_day_04_part_1(),
                "42" => solve_day_04_part_2(),
                _ => panic!("Not yet implemented")
            }
        }
    };
}
