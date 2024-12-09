use crate::days::Part;
use crate::util::read_lines;


pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let l = lines.into_iter().next().unwrap().expect("Failed to read line");
  let mut input = Vec::new();

  for ch in l.chars() {
    let n = ch as u8 - '0' as u8;
    input.push(n);
  }

  match part {
    Part::P1 => Ok(part_1(input)),
    Part::P2 => Ok(part_2(input)),
  }
}

#[derive(Debug)]
struct Block {
  len: u8,
  id: u32,
}

fn part_1(mut input: Vec<u8>) -> u64 {
  let mut total_bc = 0;

  for i in &input {
    total_bc += *i as u64;
  }

  println!("total bc: {:?}", total_bc);

  let mut formated: Vec<Block> = Vec::new();

  let mut left_i = 0;
  let mut right_i = input.len() - 1;

  println!("input: {:?}", input);

  loop {
    if left_i > right_i {
      break;
    }

    let left_is_block = left_i % 2 == 0;

    if left_is_block {
      // Current left block is a block
      // add it to the new formatedd blocks
      let block = Block {
        len: input[left_i],
        id: left_i as u32 / 2,
      };

      if block.len > 0 {
        formated.push(block);
      }

      left_i += 1;
      continue;
    }
    // Current left block is free space
    // find some block to ocupy it
    if input[left_i] == 0 {
      left_i += 1;
      continue;
    }

    let right_is_free = right_i % 2 == 1;
    if right_is_free {
      right_i -= 1;
      continue;
    }

    let free_space = input[left_i];

    let mut block = Block {
      len: input[right_i],
      id: right_i as u32 / 2,
    };

    if block.len == free_space {
      formated.push(block);
      right_i -= 1;
      left_i += 1;
    } else if block.len > free_space {
      input[right_i] = block.len - free_space;
      block.len = free_space;
      formated.push(block);
      left_i += 1;
    } else {
      input[left_i] = free_space - block.len;
      formated.push(block);
      right_i -= 1;
    }

  } 

  println!("{:?}", formated);
  checksum(formated)
}

fn checksum(formated: Vec<Block>) -> u64 {
  let mut check_sum = 0;
  let mut left_i = 0;

  for f in formated {
    for _i in 0..f.len {
      check_sum += left_i * f.id as u64;
      left_i += 1;
    }
  }

  check_sum
}

fn part_2(mut input: Vec<u8>) -> u64 {
  let mut formated: Vec<Block> = Vec::new();
  let mut moved: Vec<bool> = Vec::new();

  for _ in &input {
    moved.push(false);
  }

  let mut left_i = 0;
  let mut right_i = input.len() - 1; // rightmost block to push

  println!("input: {:?}", input);

  loop {
    if left_i >= right_i {
      break;
    }

    let right_is_free = right_i % 2 == 1;
    if right_is_free {
      right_i -= 1;
      continue;
    }

    let left_is_block = left_i % 2 == 0;
    if left_is_block {
      if moved[left_i] {
        formated.push(Block {
          len: input[left_i],
          id: 0,
        }); // add dummy to make checksome be good
        left_i += 1;
        continue;
      }
      formated.push(Block {
        len: input[left_i],
        id: left_i as u32 / 2,
      });
      moved[left_i] = true;
      left_i += 1;
      // println!("{:?}", formated);
      continue;
    }

    let mut filled = false;

    for ci in ((left_i + 1)..(right_i + 1)).rev() {
      let right_is_free = ci % 2 == 1;
      if right_is_free || moved[ci] {
        continue; // Either empty or already moved
      }

      if input[ci] > input[left_i] {
        continue; // Not enough free space
      }

      formated.push(Block {
        len: input[ci],
        id: ci as u32 / 2,
      });
      moved[ci] = true;
      input[left_i] = input[left_i] - input[ci];
      // println!("{:?}", formated);

      if input[left_i] == 0 {
        left_i += 1; // Completly filled, move onto next block
      }
      filled = true;
      break;
    }

    if !filled {
      // Unale to fill space
      formated.push(Block {
        len: input[left_i],
        id: 0
      }); // add dummy to make checksum be good
      left_i += 1;
    }
  }

  println!("{:?}", formated);
  checksum(formated)
}
