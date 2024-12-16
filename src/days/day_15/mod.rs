
use crate::days::Part;
use crate::util::read_lines;

use std::collections::BTreeSet;

#[derive(Debug)]
enum Entry {
  Space,
  Wall,
  Box
}

#[derive(Debug, Clone)]
enum Entry2 {
  Space,
  Wall,
  BoxL,
  BoxR,
}

#[derive(Debug)]
enum Move {
  Left, Right, Up, Down
}

pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut input = Vec::new();
  let mut moves = Vec::new();
  let mut start = (0, 0);

  let mut pmoves = false;

  for l in lines {
    let line = l.expect("Failed to read line");

    if line.len() == 0 {
      pmoves = true;
      continue;
    }

    if pmoves {
      for ch in line.chars() {
        let move_ = match ch {
          '<' => Move::Left,
          '>' => Move::Right,
          'v' => Move::Down,
          '^' => Move::Up,
          _ => panic!("Invalid char"),
        };
        moves.push(move_);
      }
      continue;
    }

    let mut row = Vec::new();
    for ch in line.chars() {
      let entry = match ch {
        '.' => Entry::Space,
        '#' => Entry::Wall,
        'O' => Entry::Box,
        '@' => {
          start = (input.len(), row.len());
          Entry::Space
        },
        _ => panic!("Invalid char"),
      };

      row.push(entry);
    }
    input.push(row);
  }


  match part {
    Part::P1 => Ok(part_1(input, moves, start)),
    Part::P2 => Ok(part_2(input, moves, start)),
  }
}

fn print_map(input: &Vec<Vec<Entry>>) {
  let width = input[0].len();
  let height = input.len();

  for y in 0..height {
    for x in 0..width {
      let ch = match input[y][x] {
        Entry::Wall => '#',
        Entry::Space => '.',
        Entry::Box => 'O',
      };

      print!("{}", ch);
    }
    println!();
  }
}

fn print_map_2(input: &Vec<Vec<Entry2>>, px: usize, py: usize) {
  let width = input[0].len();
  let height = input.len();
  let mut box_count = 0;

  for y in 0..height {
    for x in 0..width {
      let ch = match input[y][x] {
        Entry2::Wall => '#',
        Entry2::Space => {
          if x == px && y == py {
            '@'
          } else {
            '.'
          }
        },
        Entry2::BoxL => {
          box_count += 1;
          '['
        },
        Entry2::BoxR => ']',
      }; 


      print!("{}", ch);
    }
    println!();
  }
  println!("bc: {}", box_count);
  // if box_count != 604 {
  //   panic!()
  // }
}


fn part_1(mut input: Vec<Vec<Entry>>, moves: Vec<Move>, start: (usize, usize)) -> u64 {
  let mut x = start.1;
  let mut y = start.0;

  let width = input[0].len();
  let height = input.len();

  for dir in moves {
    // print_map(&input);
    let (dy, dx) = match dir {
      Move::Left => (0, -1),
      Move::Right => (0, 1),
      Move::Down => (1, 0),
      Move::Up => (-1, 0),
    };

    let nx = x as i32 + dx;
    let ny = y as i32 + dy;

    if nx < 0 || nx as usize >= width ||
       ny < 0 || ny as usize >= height {
      panic!("Shuoldn't be possible");
    }

    let nx = nx as usize;
    let ny = ny as usize;

    if let Entry::Space = input[ny][nx] {
      y = ny; // Easy case we can move
      x = nx;
      continue;
    };

    if let Entry::Wall = input[ny][nx] {
      // Easy case we can't move
      continue;
    };

    // Hard case we have to check if we can move boxes
    let fb_x = nx;
    let fb_y = ny;

    let mut cx = nx;
    let mut cy = ny;

    loop {
      let nx = cx as i32 + dx;
      let ny = cy as i32 + dy;

      if nx < 0 || nx as usize >= width ||
         ny < 0 || ny as usize >= height {
        panic!("Shuoldn't be possible");
      }

      let nx = nx as usize;
      let ny = ny as usize;

      if let Entry::Space = input[ny][nx] {
        // We can move all boxes
        input[fb_y][fb_x] = Entry::Space; 
        input[ny][nx] = Entry::Box;
        x = fb_x;
        y = fb_y;
        break;
      };

      if let Entry::Wall = input[ny][nx] {
        // We can't move all boxes, no update
        break;
      };

      // Found another box
      cy = ny;
      cx = nx;
    }
  }
  print_map(&input);

  let mut score = 0;

  for x in 0..width {
    for y in 0..height {
      if let Entry::Box = input[y][x] {
        score += 100 * y as u64 + x as u64;
      }
    }
  }

  score
}

fn part_2(input: Vec<Vec<Entry>>, moves: Vec<Move>, start: (usize, usize)) -> u64 {
  let mut input2 = Vec::new();
  for row in input {
    let mut r2 = Vec::new();
    for e in row {
      match e {
        Entry::Space => {
          r2.push(Entry2::Space);
          r2.push(Entry2::Space);
        },
        Entry::Wall => {
          r2.push(Entry2::Wall);
          r2.push(Entry2::Wall);
        },
        Entry::Box => {
          r2.push(Entry2::BoxL);
          r2.push(Entry2::BoxR);
        },
      }
    }
    input2.push(r2);
  }

  part_2_inner(input2, moves, (start.0, start.1 * 2))
}

fn part_2_inner(mut input: Vec<Vec<Entry2>>, moves: Vec<Move>, start: (usize, usize)) -> u64 {
  let mut x = start.1;
  let mut y = start.0;

  // to low 1511362

  let width = input[0].len();
  let height = input.len();

  for dir in moves {
    // println!("x: {}, y: {}, {:?}", x, y, &dir);
    // print_map_2(&input, x, y);
    let (dy, dx) = match dir {
      Move::Left => (0, -1),
      Move::Right => (0, 1),
      Move::Down => (1, 0),
      Move::Up => (-1, 0),
    };

    let nx = x as i32 + dx;
    let ny = y as i32 + dy;

    if nx < 0 || nx as usize >= width ||
       ny < 0 || ny as usize >= height {
      panic!("Shuoldn't be possible");
    }

    let nx = nx as usize;
    let ny = ny as usize;

    if let Entry2::Space = input[ny][nx] {
      y = ny; // Easy case we can move
      x = nx;
      println!("s");
      continue;
    };

    if let Entry2::Wall = input[ny][nx] {
      // Easy case we can't move
      println!("w");
      continue;
    };

    if dy == 0 {
      // Old style easy case movement
      let fb_x = nx;

      let mut cx = nx;
      let mut to_update = Vec::new();
      to_update.push(nx);

      loop {
        let nx = cx as i32 + dx;

        if nx < 0 || nx as usize >= width {
          panic!("Shuoldn't be possible");
        }

        let nx = nx as usize;
        to_update.push(nx);

        if let Entry2::Space = input[ny][nx] {
          // We can move all boxes
          for i in (1..to_update.len()).rev() {
            input[ny][to_update[i]] = input[ny][to_update[i - 1]].clone();
          }
          input[ny][fb_x] = Entry2::Space;
          x = fb_x;
          y = ny;
          break;
        };

        if let Entry2::Wall = input[ny][nx] {
          // We can't move all boxes, no update
          // println!("neg x: {}, y: {}, {:?}", x, y, &dir);
          // print_map_2(&input, x, y);
          break;
        };

        // Found another box
        cx = nx;
      }
      continue;
    }

    // Hard case we have to check if we can move boxes
    let mut heads = Vec::new();
    let mut to_update: BTreeSet<(usize, usize, usize)> = BTreeSet::new();

    heads.push((ny, ny, nx));
    heads.push((ny, ny, if is_left(&input[ny][nx]) {nx + 1} else {nx - 1}));
    

    loop {
      let Some((iy, sy, sx)) = heads.pop() else {
        y = ny;
        x = nx;

        // Everything is good we need to udpate all
        let mut to_update_v = to_update.into_iter().collect::<Vec<_>>();
        to_update_v.sort_by(|a, b| if dy > 0 { b.1.cmp(&a.1) } else { a.1.cmp(&b.1)} );
        // println!("{:?}", to_update_v);

        for (iy, ey, x) in to_update_v.into_iter() {
          let mut y = ey;
          // println!("iy: {}, ey: {}", iy, ey);
          loop {
            if y == iy {
              break;
            }
            let ny = (y as i32 - dy) as usize;
            input[y][x] = input[ny][x].clone();
            y = ny;
            print_map_2(&input, 0, 0);
          }
          input[y][x] = Entry2::Space;
        }

        // println!("end of update");
        break;
      };
      // println!("{:?}: x:{}, y:{}", heads, sx, sy);

      let ny = sy as i32 + dy;
      if ny < 0 || ny as usize >= height {
        panic!("Shuoldn't be possible");
      }
      let ny = ny as usize;

      if let Entry2::Space = input[ny][sx] {
        // We can move this row of boxes
        // println!("is");
        to_update.insert((iy, ny, sx));
        continue;
      };

      if let Entry2::Wall = input[ny][sx] {
        // We can't move all boxes, no update
        // println!("neg x: {}, y: {}, {:?}", x, y, &dir);
        // print_map_2(&input, x, y);
        break;
      };

      // More box found, check if introduces more heads or direct above

      if is_left(&input[sy][sx]) != is_left(&input[ny][sx]) {
        // Not directly above
        heads.push((ny, ny, if is_left(&input[ny][sx]) {sx + 1} else {sx - 1}));
        heads.push((ny, ny, sx));

        // This is also the end of a row
        to_update.insert((iy, ny, sx));
      } else {
        heads.push((iy, ny, sx));
      }
    }
  }

  print_map_2(&input, 0, 0);

  let mut score = 0;

  for x in 0..width {
    for y in 0..height {
      if let Entry2::BoxL = input[y][x] {
        score += 100 * y as u64 + x as u64;
      }
    }
  }

  score
}


fn is_left(b: &Entry2) -> bool {
  match b {
    Entry2::BoxL => true,
    Entry2::BoxR => false,
    _ => panic!()
  }
}