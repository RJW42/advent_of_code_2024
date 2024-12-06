
use crate::days::Part;
use crate::util::read_lines;

use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq)]
enum Entry {
  Wall,
  Space(bool)
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
enum Dir {
  North,
  South,
  East,
  West
}

pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut input = Vec::new();
  let mut gaurd_pos = None;
  let mut gaurd_dir = None;

  for line in lines {
    let l = line.expect("Input");
    let mut entries = Vec::new();
    for ch in l.chars() {
      let entry = match ch {
        '.' => Entry::Space(false),
        '#' => Entry::Wall,
        ch => {
          let dir = match ch {
            '^' => Dir::North,
            '>' => Dir::East,
            '<' => Dir::West,
            'v' => Dir::South,
            _ => panic!("unknown char")
          };

          gaurd_dir = Some(dir);
          gaurd_pos = Some((entries.len(), input.len()));
          Entry::Space(true)
        }
      };
      entries.push(entry);
    }

    input.push(entries);
  }

  // println!("{:?}", input);


  match part {
    Part::P1 => Ok(part_1(input, gaurd_dir.unwrap(), gaurd_pos.unwrap())),
    Part::P2 => Ok(part_2(input, gaurd_dir.unwrap(), gaurd_pos.unwrap())),
  }
}

fn part_1(mut input: Vec<Vec<Entry>>, gaurd_dir: Dir, gaurd_pos: (usize, usize)) -> u64 {
  let (mut x, mut y) = gaurd_pos;
  let mut dir = gaurd_dir;

  loop {
    let (dy, dx) = change(&dir);

    let nx = x as i32 + dx;
    let ny = y as i32 + dy;

    if nx < 0 || nx >= input[0].len() as i32 ||
       ny < 0 || ny >= input.len() as i32{
        break;
    }

    if input[ny as usize][nx as usize] == Entry::Wall {
      dir = turn(&dir);
      continue;
    }

    x = nx as usize;
    y = ny as usize;

    input[y][x] = Entry::Space(true);
  }

  let mut steps = 0;

  for row in input {
    for entry in row {
      if let Entry::Space(true) = entry {
        steps += 1;
      }
    }
  }

  steps
}

fn part_2(mut input: Vec<Vec<Entry>>, gaurd_dir: Dir, gaurd_pos: (usize, usize)) -> u64 {
  let mut prev_positions = Vec::new();

  for _y in 0..input.len() {
    let mut row = Vec::new();
    for _x in 0..input[0].len() {
      row.push(BTreeSet::new());
    }
    prev_positions.push(row);
  }

  let (mut x, mut y) = gaurd_pos;
  let mut dir = gaurd_dir;
  prev_positions[y][x].insert(dir.clone());

  let mut can_cause_loop = BTreeSet::new();

  loop {
    let (dy, dx) = change(&dir);

    let nx = x as i32 + dx;
    let ny = y as i32 + dy;

    if nx < 0 || nx >= input[0].len() as i32 ||
       ny < 0 || ny >= input.len() as i32 {
        break;
    }

    if input[ny as usize][nx as usize] == Entry::Wall {
      dir = turn(&dir);
      prev_positions[y][x].insert(dir.clone());
      continue;
    }

    if let Entry::Space(true)  = input[ny as usize][nx as usize] {
      // Can't placea block here, needed to get here
      x = nx as usize;
      y = ny as usize;

      prev_positions[y][x].insert(dir.clone());
      continue;
    }

    let mut inner_prev_positions = Vec::new();
    for _y in 0..input.len() {
      let mut row = Vec::new();
      for _x in 0..input[0].len() {
        row.push(BTreeSet::new());
      }
      inner_prev_positions.push(row);
    }

    let mut ix = x;
    let mut iy = y;
    let mut idir = dir.clone();
    inner_prev_positions[iy][ix].insert(idir.clone());

    let bx = nx as usize;
    let by = ny as usize;

    input[ny as usize][nx as usize] = Entry::Wall;

    loop {
      let (dy, dx) = change(&idir);

      let nx = ix as i32 + dx;
      let ny = iy as i32 + dy;

      if nx < 0 || nx >= input[0].len() as i32 ||
         ny < 0 || ny >= input.len() as i32 {
          break;
      }

      if input[ny as usize][nx as usize] == Entry::Wall {
        idir = turn(&idir);
        inner_prev_positions[iy][ix].insert(idir.clone());
        continue;
      }

      iy = ny as usize;
      ix = nx as usize;
      
      if inner_prev_positions[iy][ix].contains(&idir) || 
         prev_positions[iy][ix].contains(&idir) {
        can_cause_loop.insert(bx * input[0].len() + by);
        break;
      }
      inner_prev_positions[iy][ix].insert(idir.clone());
    }
    
    input[ny as usize][nx as usize] = Entry::Space(true);

    x = nx as usize;
    y = ny as usize;

    prev_positions[y][x].insert(dir.clone());
  }

  can_cause_loop.len() as u64
}

fn change(dir: &Dir) -> (i32, i32) {
  match dir {
    Dir::North => (-1, 0),
    Dir::South => (1, 0),
    Dir::East => (0, 1),
    Dir::West => (0, -1),
  } 
}

fn turn(dir: &Dir) -> Dir {
  match dir {
    Dir::North => Dir::East,
    Dir::East => Dir::South,
    Dir::South => Dir::West,
    Dir::West => Dir::North,
  } 
}