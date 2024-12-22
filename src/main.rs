mod days;

pub mod util;

use days::*;
use days::Part::{P1, P2};


fn main() {
    let day = 22;

    let function = match day {
        1 => |p| day_1::run("inputs/day_1.txt", p),
        2 => |p| day_2::run("inputs/day_2.txt", p),
        3 => |p| day_3::run("inputs/day_3.txt", p),
        4 => |p| day_4::run("inputs/day_4.txt", p),
        5 => |p| day_5::run("inputs/day_5.txt", p),
        6 => |p| day_6::run("inputs/day_6.txt", p),
        7 => |p| day_7::run("inputs/day_7.txt", p),
        8 => |p| day_8::run("inputs/day_8.txt", p),
        9 => |p| day_9::run("inputs/day_9.txt", p),
        10 => |p| day_10::run("inputs/day_10.txt", p),
        11 => |p| day_11::run("inputs/day_11.txt", p),
        12 => |p| day_12::run("inputs/day_12.txt", p),
        13 => |p| day_13::run("inputs/day_13.txt", p),
        14 => |p| day_14::run("inputs/day_14.txt", p),
        15 => |p| day_15::run("inputs/day_15.txt", p),
        16 => |p| day_16::run("inputs/day_16.txt", p),
        17 => |p| day_17::run("inputs/day_17.txt", p),
        18 => |p| day_18::run("inputs/day_18.txt", p),
        19 => |p| day_19::run("inputs/day_19.txt", p),
        20 => |p| day_20::run("inputs/day_20.txt", p),
        21 => |p| day_21::run("inputs/day_21.txt", p),
        22 => |p| day_22::run("inputs/day_22.txt", p),
        _ => |_| Result::Err("Invalid Day"),
    };

    let result = function(P1)
        .map_err(|e| format!("Failed P1: {e}"))
        .and_then(|r| {
            println!("P1 Result: {}", r);
            return function(P2)
                .map_err(|e| format!("Failed P2: {e}"))
                .map(|r| println!("P2 Results: {}", r));
        });

    match result {
        Ok(_) => (),
        Err(reason) => println!("{}", reason)
    };
}