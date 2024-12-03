pub mod day_1;
pub mod day_2;

use std::iter::Peekable;

pub enum Part {
    P1,
    P2
}

pub fn parse_num<'a, I>(chars: &mut Peekable<I>, skip_non_numeric: bool) -> Option<i64> 
where I: Iterator<Item = &'a char> {
    let mut output = None;
    let mut output_val = 0;
    let mut sign = 1;

    loop {
        match chars.peek() {
            Some('-') => {
                sign = -1;
                chars.next();
            },
            ch @ Some('0'..='9') => {
                output_val = output_val * 10 + (
                    **ch.unwrap() as i64 - '0' as i64
                );
                output = Some(output_val);
                chars.next();
            },
            None => break,
            _ => {
                if skip_non_numeric && output == None {
                    chars.next();
                    continue
                } else {
                    break;
                }
            },
        };
    }

    output.map(|v| v * sign)
}