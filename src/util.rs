use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::iter::Peekable;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


pub fn parse_num<'a, I>(
    chars: &mut Peekable<I>, 
    skip_non_numeric: bool,
    allow_negative: bool,
) -> Option<i64> 
where I: Iterator<Item = &'a char> {
    let mut output = None;
    let mut output_val = 0;
    let mut sign = 1;

    loop {
        match chars.peek() {
            Some('-') => {
                if !allow_negative {
                    break;
                }
                // TODO: bug here in that -10-1 == 101? 
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