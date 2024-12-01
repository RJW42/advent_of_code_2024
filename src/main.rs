mod days;

pub mod util;

use days::*;
use days::Part::{P1, P2};


fn main() {
    let day = 1;

    let function = match day {
        1 => |p| day_1::run("inputs/day_1.txt", p),
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