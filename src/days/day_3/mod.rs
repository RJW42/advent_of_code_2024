
use crate::days::Part;
use crate::util::{read_lines, parse_num};
use std::iter::Peekable;

#[derive(Debug)]
struct Mul {
  x: u16,
  y: u16,
}

pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut input = String::new();
  
  for line in lines {
    let l = line.expect("Input");
    input += l.as_str();
  }


  match part {
    Part::P1 => Ok(part_1(input)),
    Part::P2 => Ok(part_2(input)),
  }
}


fn part_1(line: String) -> u64 {
  let chars_vec = line.chars().collect::<Vec<char>>();
  let mut chars = chars_vec.iter().peekable();
  let mut muls = Vec::new();

  loop {
    if let None = chars.peek() {
      break;
    }

    if let Some(mul) = can_parse_mul(&mut chars){
      muls.push(mul);
    } else {
      let _ = chars.next();
    }
  }

  println!("{:?}", muls);

  muls.into_iter()
      .map(|m| m.x as u64 * m.y as u64)
      .reduce(|acc, x| acc + x)
      .expect("No muls")
}

fn can_parse_do<'a, I>(
  chars: &mut Peekable<I>, 
) -> bool 
  where I: Iterator<Item = &'a char> 
{
    if !can_parse(chars, 'd') { return false; }
    if !can_parse(chars, 'o') { return false; }
    if !can_parse(chars, '(') { return false; }
    if !can_parse(chars, ')') { return false; }

    return true;
}

fn can_parse_dont<'a, I>(
  chars: &mut Peekable<I>, 
) -> bool 
  where I: Iterator<Item = &'a char> 
{
    if !can_parse(chars, 'd') { return false; }
    if !can_parse(chars, 'o') { return false; }
    if !can_parse(chars, 'n') { return false; }
    if !can_parse(chars, '\'') { return false; }
    if !can_parse(chars, 't') { return false; }
    if !can_parse(chars, '(') { return false; }
    if !can_parse(chars, ')') { return false; }

    return true;
}

fn can_parse_mul<'a, I>(
  chars: &mut Peekable<I>, 
) -> Option<Mul> 
  where I: Iterator<Item = &'a char> 
{
    if !can_parse(chars, 'm') { return None; }
    if !can_parse(chars, 'u') { return None; }
    if !can_parse(chars, 'l') { return None; }
    if !can_parse(chars, '(') { return None; }
    let x = parse_num(chars, false, false)?;
    if !can_parse(chars, ',') { return None; }
    let y = parse_num(chars, false, false)?;
    if !can_parse(chars, ')') { return None; }

    Some(Mul {
      x: x as u16,
      y: y as u16
    })
}

fn can_parse<'a, I>(
  chars: &mut Peekable<I>, 
  expected: char,
) -> bool  
  where I: Iterator<Item = &'a char> 
{
  let output = if let Some(val) = chars.peek() {
    *(*val) == expected
  } else { false };

  if output {
    let _ = chars.next();
  }

  return output;
}

fn part_2(line: String) -> u64 {
  let chars_vec = line.chars().collect::<Vec<char>>();
  let mut chars = chars_vec.iter().peekable();
  let mut muls = Vec::new();
  let mut in_dont = false;

  loop {
    if let None = chars.peek() {
      break;
    }

    if !in_dont {
      if let Some(mul) = can_parse_mul(&mut chars){
        muls.push(mul);
      } else if can_parse_dont(&mut chars) {
        in_dont = true;
      } else {
        let i = chars.next();
      }
    } else {
      if can_parse_do(&mut chars) {
        in_dont = false;
      } else {
        let _ = chars.next();
      }
    }
  }

  println!("{:?}", muls);

  muls.into_iter()
      .map(|m| m.x as u64 * m.y as u64)
      .reduce(|acc, x| acc + x)
      .expect("No muls")
}