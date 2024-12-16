
use crate::days::Part;
use crate::util::read_lines;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Dir {
  North,
  East,
  South,
  West,
}

pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut map = Vec::new();
  let mut start = (0, 0);
  let mut goal = (0, 0);

  for l in lines {
    let line = l.expect("Failed to read line");
    let mut row = Vec::new();
    for ch in line.chars() {
      let is_wall = match ch {
        '.' => false,
        '#' => true,
        'S' => {
          start = (map.len(), row.len());
          false
        },
        'E' => {
          goal = (map.len(), row.len());
          false
        },
        _ => panic!("Invalid character")
      };
      row.push(is_wall);
    }
    map.push(row);
  }

  match part {
    Part::P1 => Ok(part_1(map, start, goal, false)),
    Part::P2 => Ok(part_1(map, start, goal, true)),
  }
}

fn di(dir: &Dir) -> usize {
  match dir {
    Dir::East => 0,
    Dir::West => 1,
    Dir::North => 2,
    Dir::South => 3,
  }
}

fn moves(dir: &Dir) -> &[(Dir, i32, i32, u32); 4] {
  const E_MOVES: [(Dir, i32, i32, u32); 4] = [(Dir::East, 0,  1, 1), (Dir::West, 0, -1, 2001), (Dir::South, 1, 0, 1001), (Dir::North, -1, 0, 1001)];
  const W_MOVES: [(Dir, i32, i32, u32); 4] = [(Dir::West, 0, -1, 1), (Dir::East, 0,  1, 2001), (Dir::South, 1, 0, 1001), (Dir::North, -1, 0, 1001)];
  const N_MOVES: [(Dir, i32, i32, u32); 4] = [(Dir::North, -1, 0, 1), (Dir::South,  1, 0, 2001), (Dir::East, 0, 1, 1001), (Dir::West, 0, -1, 1001)];
  const S_MOVES: [(Dir, i32, i32, u32); 4] = [(Dir::South,  1, 0, 1), (Dir::North, -1, 0, 2001), (Dir::East, 0, 1, 1001), (Dir::West, 0, -1, 1001)];

  match dir {
    Dir::East => &E_MOVES,
    Dir::West => &W_MOVES,
    Dir::North => &N_MOVES,
    Dir::South => &S_MOVES,
  }
}

fn print_map(map: &Vec<Vec<bool>>, path: &Vec<Vec<usize>>) {
  for y in 0..map.len() {
    for x in 0..map[0].len() {
      let ch = if map[y][x] {
        '#'
      } else {
        match path[y][x] {
          0 => '>',
          1 => '<',
          2 => '^',
          3 => 'v',
          _ => '.',
        }
      };
      print!("{}", ch);
    }
    println!();
  }
}


fn part_1(
  map: Vec<Vec<bool>>, 
  (sy, sx): (usize, usize), 
  (gy, gx): (usize, usize), 
  return_len: bool
) -> u64 {

  let mut gscore = Vec::new();
  let mut fscore = Vec::new();
  let mut to_visit = Vec::new();
  let mut previous: Vec<Vec<[Vec<(usize, usize, usize)>; 4]>> = Vec::new();
  let mut path = Vec::new();

  for r in &map {
    gscore.push(vec![u32::MAX; r.len() * 4]);
    fscore.push(vec![u32::MAX; r.len() * 4]);
    path.push(vec![5; r.len()]);
    previous.push(vec![[Vec::new(), Vec::new(), Vec::new(), Vec::new()]; r.len()]);
  }

  let si = |x, i| {x * 4 + i};

  gscore[sy][si(sx, 0)] = (sy.abs_diff(gy) + sx.abs_diff(gx)) as u32;
  fscore[sy][si(sx, 0)] = 0;
  to_visit.push((Dir::East, sy, sx));

  println!("s: {} {}", sx, sy);
  println!("g: {} {}", gx, gy);

  loop {
    to_visit.sort_by(|b, a| gscore[a.1][si(a.2, di(&a.0))].cmp(&gscore[b.1][si(b.2, di(&b.0))]));
    // println!("{:?}", to_visit);
    let Some((dir, y, x)) = to_visit.pop() else {
      let mut min = u32::MAX;
      let mut queue = Vec::new();
      let mut len = 0;
      for i in 0..4 {
        let s = fscore[gy][si(gx, i)];
        if s > min {
          continue;
        } else if s == min {
          queue.push((gy, gx, i));
          continue;
        } 
        queue.clear();
        queue.push((gy, gx, i));
        min = s;
      }
      loop {
        let Some((y, x, di)) = queue.pop() else {
          break;
        };
        path[y][x] = di;
        if x == sx && y == sy {
          continue;
        }
        println!(" - x: {}, y: {}, d: {}", x, y, di);
        for element in &previous[y][x][di] {
          queue.push(element.clone());
        }
      }
      for r in &path {
        for e in r {
          if *e != 5 {
            len += 1;
          }
        }
      }
      print_map(&map, &path);
      return if return_len {len} else {min as u64};
    };
    let cdi = di(&dir);
    let xi = si(x, cdi);

    if y == gy && x == gx {
      println!("goal {} {} {:?} {}", x, y, dir, fscore[gy][xi]);
      continue;
      // return fscore[gy][xi] as u64;
    }


    for (d_dir, dy, dx, cost) in moves(&dir) {
      let nx = x as i32 + dx;
      let ny = y as i32 + dy;
      let ndi = di(d_dir);

      if nx < 0 || nx as usize >= map[0].len() ||
         ny < 0 || ny as usize >= map.len() {
        continue;
      }
      
      let nx = nx as usize;
      let ny = ny as usize;

      if map[ny][nx] {
        continue;
      }

      let nxi = si(nx, ndi);

      let score = fscore[y][xi] + cost;
      if score > fscore[ny][nxi] {
        continue;
      } else if score == fscore[ny][nxi] {
        previous[ny][nx][ndi].push((y, x, cdi));
        continue;
      }
      
      fscore[ny][nxi] = score;
      gscore[ny][nxi] = score + (ny.abs_diff(gy) + nx.abs_diff(gx)) as u32;
      previous[ny][nx][ndi].clear();
      previous[ny][nx][ndi].push((y, x, cdi));

      let element = (*d_dir, ny, nx);

      if !to_visit.contains(&element) {
        to_visit.push(element);
      }
    }
  }
}