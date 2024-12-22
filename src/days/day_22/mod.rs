
use crate::days::Part;
use crate::util::read_lines;

use std::collections::BTreeMap;

const MOD: u64 = 16777216;

pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut nums = Vec::new();

  for l in lines {
    let line = l.expect("Failed to read line");
    nums.push(line.parse::<u64>().expect("failed to parse num"));
  }

  match part {
    Part::P1 => Ok(part_1(nums)),
    Part::P2 => Ok(part_2(nums)),
  }
}

fn part_1(nums: Vec<u64>) -> u64 {
  const AMOUNT: u32 = 2000;

  let mut sum = 0;
 
  for num in nums {
    // print!("{}: ", num);
    let mut secrets = num;
    for _i in 0..AMOUNT {
      secrets = next_secret(secrets);
    }
    // println!("{:?}", secrets);
    sum += secrets as u64;
  }

  sum
}

fn part_2(nums: Vec<u64>) -> u64 {
  const AMOUNT: usize = 2000;

  let mut total_scores = BTreeMap::new();
 
  for num in nums {
    let mut secrets = num;
    let mut diffs: [i8; 4] = [0i8; 4];
    let mut scores = BTreeMap::new();
    for i in 0..AMOUNT {
      let d1 = secrets % 10;
      secrets = next_secret(secrets);
      let d2 = secrets % 10;
      let diff = (d2 as i8) - (d1 as i8);
      if i < 3 {
        diffs[i % 4] = diff;
        continue;
      } else if i == 3 {
        diffs[3] = diff as i8;
      } else {
        diffs[0] = diffs[1];
        diffs[1] = diffs[2];
        diffs[2] = diffs[3];
        diffs[3] = diff as i8;
      }
      if !scores.contains_key(&diffs) {
        scores.insert(diffs.clone(), d2);
      }
    }
    for (key, val) in scores {
      if !total_scores.contains_key(&key) {
        total_scores.insert(key, val as u64);
      } else {
        *total_scores.get_mut(&key).unwrap() += val as u64;
      }
    }
  }

  let mut max = 0;
  for (_key, val) in total_scores {
    max = u64::max(max, val);
  }

  max
}

fn next_secret(mut secrets: u64) -> u64 {
  secrets = mix(secrets, secrets * 64);
  secrets = prune(secrets);
  secrets = mix(secrets, secrets / 32);
  secrets = prune(secrets);
  secrets = mix(secrets, secrets * 2048);
  secrets = prune(secrets);
  return secrets;
}

fn mix(secret: u64, value: u64) -> u64 {
  secret ^ value
}

fn prune(secret: u64) -> u64 {
  secret % MOD
}