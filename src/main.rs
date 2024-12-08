mod days;

pub mod util;

use days::*;
use days::Part::{P1, P2};


fn main() {
    let day = 7;

    let function = match day {
        1 => |p| day_1::run("inputs/day_1.txt", p),
        2 => |p| day_2::run("inputs/day_2.txt", p),
        3 => |p| day_3::run("inputs/day_3.txt", p),
        4 => |p| day_4::run("inputs/day_4.txt", p),
        5 => |p| day_5::run("inputs/day_5.txt", p),
        6 => |p| day_6::run("inputs/day_6.txt", p),
        7 => |p| day_7::run("inputs/day_7.txt", p),
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