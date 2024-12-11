

use crate::days::Part;
use crate::util::read_lines;

use std::collections::BTreeMap;


pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };
  let l = lines.into_iter().next().unwrap().expect("unable to read line");

  let input = l.split(" ")
    .map(|n| n.parse::<u64>().expect("Failed"))
    .collect();

  match part {
    Part::P1 => Ok(part_1(input, 25)),
    Part::P2 => Ok(part_1(input, 75)),
  }
}

fn part_1(input: Vec<u64>, itters: u64) -> u64 {
  let mut ns = BTreeMap::new();

  for n in input {
    insert_or_add(&mut ns, n, 1);
  }

  for _i in 0..itters {
    // println!("{:?}", ns);
    let mut tmp = BTreeMap::new();

    for (n, count) in ns {
      if n == 0 { // 0 -> 1
        insert_or_add(&mut tmp, 1, count);
        continue;
      }
      let num_digits = count_digits(n);
      if num_digits % 2 == 1 { // non even mul 2024
        insert_or_add(&mut tmp, n * 2024, count);
        continue;
      } // even digits, split
      let mut n1 = 0;
      let mut n2 = 0;
      let mut pow = 10u64.pow(num_digits as u32 - 1 as u32);
      let mut x = n;

      for i in 0..num_digits {
        let digit = x / pow; 
        x = x % pow;
        pow /= 10;
        if i < num_digits / 2 {
          n1 = n1 * 10 + digit;
        } else {
          n2 = n2 * 10 + digit;
        }
      }
      insert_or_add(&mut tmp, n2, count);
      insert_or_add(&mut tmp, n1, count);
    }

    ns = tmp;
  }

  let mut count = 0;

  for (_key, c) in ns {
    count += c as u64;
  }

  count
}

fn insert_or_add(ns: &mut BTreeMap<u64, u64>, key: u64, count: u64) {
    if ns.contains_key(&key) {
      *ns.get_mut(&key).unwrap() += count;
      return;
    }
    ns.insert(key, count);
}

fn count_digits(n: u64) -> u64 {
  let mut power = 10;
  let mut count = 1;
  while n >= power {
      count += 1;
      if let Some(new_power) = power.checked_mul(10) {
          power = new_power;
      } else {
          break;
      }
  }
  count
}
