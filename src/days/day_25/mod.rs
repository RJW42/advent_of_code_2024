
use crate::days::Part;
use crate::util::read_lines;


pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut keys = Vec::new();
  let mut locks = Vec::new();

  let mut i = 0;
  let mut curr = [[false; 5]; 7];
  for l in lines {
    let line = l.expect("Failed to read line");
    if i == 7 {
      i = 0;
      add(&mut keys, &mut locks, &curr);
      continue;
    }

    for (j, ch) in line.chars().into_iter().enumerate() {
      curr[i][j] = match ch {
        '#' => true,
        '.' => false,
        _ => panic!("Invalid charr")
      };
    }
    i += 1;
  }
  add(&mut keys, &mut locks, &curr);

  // println!("keys: {:?}", keys);
  // println!("locks: {:?}", locks);

  match part {
    Part::P1 => Ok(part_1(keys, locks)),
    Part::P2 => Ok(part_1(keys, locks)),
  }
}

fn part_1(
  keys: Vec<Vec<u8>>, 
  locks: Vec<Vec<u8>>,
) -> u64 {
  let mut count = 0;

  for k in keys {
    for l in &locks {
      let mut all_match = true;
      for i in 0..5 {
        if l[i] + k[i] > 5 {
          all_match = false;
          break;
        }
      }
      count += all_match as u64;
    }
  }

  count
}

fn add(
  keys: &mut Vec<Vec<u8>>, 
  locks: &mut Vec<Vec<u8>>,
  curr: &[[bool; 5]; 7]
) {
  let mut element = Vec::new();
  for j in 0..5 {
    let mut sum = 0;
    for i in 0..7 {
      sum += curr[i][j] as u8;
    }
    element.push(sum - 1);
  }
  if curr[0][0] {
    locks.push(element);
  } else {
    keys.push(element);
  }
}
