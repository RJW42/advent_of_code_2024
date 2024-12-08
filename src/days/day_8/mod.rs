use crate::days::Part;
use crate::util::read_lines;

use std::collections::BTreeMap;

pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut width = 0;
  let mut height = 0;
  let mut input = BTreeMap::new();

  for line in lines {
    let l = line.expect("Input");

    width = l.len() as u32;

    for (x, ch) in l.chars().enumerate() {
      if ch == '.' {
        continue;
      }

      if !input.contains_key(&ch) {
        input.insert(ch, Vec::new());
      }

      input.get_mut(&ch).expect("jmmmm").push((x as u32, height));
    }

    height += 1;
  }

  match part {
    Part::P1 => Ok(part_1(input, width, height)),
    Part::P2 => Ok(part_2(input, width, height)),
  }
}


fn part_1(input: BTreeMap<char, Vec<(u32, u32)>>, width: u32, height: u32) -> u64 {
  let mut nodes = BTreeMap::new();

  for (_freq, antenas) in input.iter() {
    for i in 0..antenas.len() {
      for j in 0..antenas.len() {
        if i == j {
          continue;
        }

        let (a1x, a1y) = antenas[i];
        let (a2x, a2y) = antenas[j];

        let dx = a1x as i32 - a2x as i32;
        let dy = a1y as i32 - a2y as i32;

        let nx = a1x as i32 + dx;
        let ny = a1y as i32 + dy;

        if nx < 0 || nx as u32 >= width ||
           ny < 0 || ny as u32 >= height {
            continue;
        }

        let pair = (nx as u32, ny as u32);

        if !nodes.contains_key(&pair) {
          nodes.insert(pair, Vec::new());
        }
        nodes.get_mut(&pair).expect("hmm").push((_freq, i, j));
      }
    }
  }

  // println!("{:?}", input);
  // println!("{:?}", nodes);

  nodes.len() as u64
}


fn part_2(input: BTreeMap<char, Vec<(u32, u32)>>, width: u32, height: u32) -> u64 {
  let mut nodes = BTreeMap::new();

  for (_freq, antenas) in input.iter() {
    for i in 0..antenas.len() {
      for j in 0..antenas.len() {
        if i == j {
          continue;
        }

        let (a1x, a1y) = antenas[i];
        let (a2x, a2y) = antenas[j];

        let dx = a1x as i32 - a2x as i32;
        let dy = a1y as i32 - a2y as i32;

        let mut cx = a2x;
        let mut cy = a2y;

        loop {
          let nx = cx as i32 + dx;
          let ny = cy as i32 + dy;

          if nx < 0 || nx as u32 >= width ||
             ny < 0 || ny as u32 >= height {
              break;
          }

          let pair = (nx as u32, ny as u32);

          cx = pair.0;
          cy = pair.1;

          if !nodes.contains_key(&pair) {
            nodes.insert(pair, Vec::new());
          }
          nodes.get_mut(&pair).expect("hmm").push((_freq, i, j));
        }
      }
    }
  }

  // println!("{:?}", input);
  // println!("{:?}", nodes);

  nodes.len() as u64
}