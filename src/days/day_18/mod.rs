
use crate::days::Part;
use crate::util::read_lines;


pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut input = Vec::new();

  for l in lines {
    let line = l.expect("Faield to read line");
    let (xs, ys) = line.split_once(",").unwrap();
    let x = xs.parse::<u16>().expect("Failed to parse x");
    let y = ys.parse::<u16>().expect("Failed to parse y");
    input.push((y, x));
  }

  let width = 71;
  let height = 71;
  let time = 1024;

  match part {
    Part::P1 => Ok(part_1(&input, width, height, time).unwrap()),
    Part::P2 => Ok(part_2(input, width, height, time)),
  }
}


fn part_1(input: &Vec<(u16, u16)>, width: usize, height: usize, time: usize) -> Option<u64> {
  const DIRS: [(i16, i16); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
  let mut map: Vec<Vec<bool>> = Vec::new();
  let mut gscore = Vec::new();
  let mut fscore = Vec::new();
  let mut to_visit: Vec<(u16, u16)> = Vec::new();

  let gx = width - 1;
  let gy = height - 1;

  for _y in 0..height {
    map.push(vec![false; width]);
    gscore.push(vec![u32::MAX; width]);
    fscore.push(vec![u32::MAX; width]);
  }

  for i in 0..(usize::min(input.len(), time)) {
    let (y, x) = *(&input[i]);
    map[y as usize][x as usize] = true;
  }

  gscore[0][0] = 0;
  fscore[0][0] = (width + height) as u32;
  to_visit.push((0, 0));

  loop {
    to_visit.sort_by(|b, a| fscore[a.0 as usize][a.1 as usize].cmp(&fscore[b.0 as usize][b.1 as usize]));
    // println!("{:?}", to_visit);
    let Some((y, x)) = to_visit.pop() else {
      return None;
    };

    if y as usize == gy && x as usize == gx {
      return Some(gscore[gy][gx] as u64);
    }

    for (dy, dx) in DIRS {
      let nx = x as i16 + dx;
      let ny = y as i16 + dy;

      if nx < 0 || nx as usize >= width ||
         ny < 0 || ny as usize >= height {
        continue;
      }

      let nx = nx as u16;
      let ny = ny as u16;

      if map[ny as usize][nx as usize] {
        continue;
      }

      let score = gscore[y as usize][x as usize] + 1;
      if score >= gscore[ny as usize][nx as usize] {
        continue;
      }

      fscore[ny as usize][nx as usize] = score + (width.abs_diff(nx as usize) + height.abs_diff(ny as usize)) as u32;
      gscore[ny as usize][nx as usize] = score;

      let element = (ny, nx);
      if !to_visit.contains(&element) {
        to_visit.push(element);
      }
    }


  }
}

fn part_2(input: Vec<(u16, u16)>, width: usize, height: usize, s_time: usize) -> u64 {
  let mut curr_time = s_time;
  let mut max_time = input.len();

  loop {
    if max_time - curr_time <= 1 {
      println!("solved: {}, {}", max_time, curr_time);
      println!("solution: {},{}", input[curr_time].1, input[curr_time].0);
      return 0;
    }

    let check = curr_time + ((max_time - curr_time) / 2);
    // println!("checking: {}", check);
    if let Some(_dist) = part_1(&input, width, height, check) {
      // println!("good");
      curr_time = check;
    } else {
      // println!("bad");
      max_time = check;
    }
  }
}