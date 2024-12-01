use std::collections::HashMap;

use crate::days::Part;
use crate::util::read_lines;




pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
   let Ok(lines) = read_lines(file_name) else {
      return Err("Failed to read lines");
   };

   let (left, right): (Vec<u32>, Vec<u32>) = lines.into_iter()
      .map(|line| {
         let string = line.expect("unwable to read line");
         let (a, b) = string.split_once("   ").unwrap();
         (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
   }).unzip();

   match part {
      Part::P1 => Ok(part_1(left, right)),
      Part::P2 => Ok(part_2(left, right)),
   }
}


fn part_1(mut left: Vec<u32>, mut right: Vec<u32>) -> u64 {
   left.sort();
   right.sort();

   let sum = left.into_iter()
      .zip(right.into_iter())
      .map(|(l, r)| l.abs_diff(r))
      .reduce(|a, b| a + b)
      .expect("Unable to sum");

   sum as u64
}


fn part_2(left: Vec<u32>, right: Vec<u32>) -> u64 {
   let mut map = HashMap::new();

   for n in right {
      if map.contains_key(&n) {
         *(map.get_mut(&n).unwrap()) += 1;
      } else {
         map.insert(n, 1);
      }
   }

   let mut sum = 0;

   for n in left {
      if map.contains_key(&n) {
         sum += map.get(&n).unwrap() * n;
      }
   }

   sum as u64
}