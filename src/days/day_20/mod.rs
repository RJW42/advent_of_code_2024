
use crate::days::Part;
use crate::util::read_lines;

pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut map = Vec::new();
  let mut start_pos = (0, 0);
  let mut end_pos = (0, 0);

  for l in lines {
    let line = l.expect("Failed to read line");
    let mut row = Vec::new();
    for ch in line.chars() {
      let is_wall = match ch {
        '.' => false,
        '#' => true,
        'S' => {
          start_pos = (map.len(), row.len());
          false
        },
        'E' => {
          end_pos = (map.len(), row.len());
          false
        },
        _ => panic!("Invalid character gound"),
      };

      row.push(is_wall);
    }
    map.push(row);
  }

  match part {
    Part::P1 => Ok(part_2(map, start_pos, end_pos, 100, 2)),
    Part::P2 => Ok(part_2(map, start_pos, end_pos, 100, 20)),
  }
}

fn part_2(
  map: Vec<Vec<bool>>, 
  (sy, sx): (usize, usize), 
  (ey, ex): (usize, usize),
  min_save: u32,
  cheat_size: usize,
) -> u64 {
  let (score, prev) = a_star(&map, (sy, sx), (ey, ex), 0);
  let mut path = Vec::new();

  println!("{}", score[ey][ex]);
  let mut x = ex;
  let mut y = ey;
  loop {
    path.push((y, x));
    if x == sx && y == sy {
      break;
    }
    let p = prev[y][x];
    y = p.0;
    x = p.1;
    if y == 0 && x == 0 {
      panic!("hmm");
    }
  }
  println!("path build ms {}, cs: {}, pl: {}", min_save, cheat_size, path.len());

  let mut count = 0;

  for i in 0..path.len() {
    for j in (i + 1)..path.len() {
      let dist = path[i].0.abs_diff(path[j].0) + path[i].1.abs_diff(path[j].1);
      if dist > cheat_size {
        continue;
      }

      let cost = (j - i) as u32;
      if cost - (dist as u32) < min_save {
        continue;
      }

      count += 1;
    }
  }

  count
}

// fn part_1(
//   map: Vec<Vec<bool>>, 
//   (sy, sx): (usize, usize), 
//   (ey, ex): (usize, usize),
//   min_save: u32,
//   cheat_size: usize
// ) -> u64 {
//   let xi_f = |x: usize, z: usize| {x * (cheat_size + 1) + z};
//   let width = map[0].len();
//   let height = map.len();

//   let (forward_score, _f_prev) = a_star(&map, (sy, sx), (ey, ex), cheat_size);
//   let (reverse_score, _r_prev) = a_star(&map, (ey, ex), (sy, sx), cheat_size);

//   let best_n_cheat = forward_score[ey][xi_f(ex, 0)];

//   let mut count = 0;

//   for y in 0..height {
//     for x in 0..width {
//       let f_score = forward_score[y][xi_f(x, 1)];
//       let r_score = reverse_score[y][xi_f(x, 1)];
      
//       if f_score == u32::MAX || r_score == u32::MAX || 
//         f_score + r_score > best_n_cheat - min_save {
//         continue;
//       }

//       count += 1;
//     }
//   }
//   count
// }

fn a_star(
  map: &Vec<Vec<bool>>, 
  (sy, sx): (usize, usize), 
  (ey, ex): (usize, usize),
  cheat_size: usize
) -> (Vec<Vec<u32>>, Vec<Vec<(usize, usize, usize)>>) {
  const DIRS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

  // For each position on map, add 4 points, 0 -> deafult, 1,2 -> using cheat, 3 -> used cheat
  let mut fscore = Vec::new();
  let mut gscore = Vec::new();
  let mut previous = Vec::new();
  let mut queue = Vec::new();

  let width = map[0].len();
  let height = map.len();
  let xi_f = |x: usize, z: usize| {x * (cheat_size + 1) + z};

  for _y in 0..height  {
    fscore.push(vec![u32::MAX; width * (cheat_size + 1)]);
    gscore.push(vec![u32::MAX; width * (cheat_size + 1)]);
    previous.push(vec![(0, 0, 0); width * (cheat_size + 1)]);
  }

  queue.push((sy, sx, 0));

  gscore[sy][xi_f(sx, 0)] = 0;
  fscore[sy][xi_f(sx, 0)] = ey.abs_diff(sy) as u32 + ex.abs_diff(sx) as u32;

  loop {
    queue.sort_by(|a, b| fscore[b.0][xi_f(b.1, b.2)].cmp(&fscore[a.0][xi_f(a.1, a.2)]));
    let Some((y, x, z)) = queue.pop() else {
      break;
    };

    if y == ey && x == ex {
      println!("found end");
      continue;
    }
    let score = gscore[y][xi_f(x, z)] + 1;

    for (dy, dx) in DIRS {
      let nx = x as i32 + dx;
      let ny = y as i32 + dy;

      if nx < 0 || nx as usize >= width ||
         ny < 0 || ny as usize >= height {
        continue;
      }

      let nx = nx as usize;
      let ny = ny as usize;

      let cheat = z < cheat_size;
      let used_cheat = z > 0;

      let mut moves = Vec::new();

      if map[ny][nx] {
        if cheat {
          // easy in wall need to keep cheeting
          moves.push((ny, nx, z + 1));
        }
      } else if used_cheat {
        // can either stop cheating or continue using cheats for more walls
        moves.push((ny, nx, cheat_size));
        if cheat {
          moves.push((ny, nx, z + 1));
        }
      } else {
        // no wall, not using cheats but can start
        moves.push((ny, nx, 0));
        if 1 < cheat_size {
          moves.push((ny, nx, 1));
        }
      }

      for (ny, nx, nz) in moves {
        if score >= gscore[ny][xi_f(nx, nz)] {
          continue;
        }

        previous[ny][xi_f(nx, nz)] = (y, x, z);
        gscore[ny][xi_f(nx, nz)] = score;
        fscore[ny][xi_f(nx, nz)] = score + (ny.abs_diff(ey) as u32 + nx.abs_diff(ex) as u32);

        let element = (ny, nx, nz);
        if !queue.contains(&element) {
          queue.push(element);
        }
      }
    }
  }

  (gscore, previous)
}