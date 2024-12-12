
use crate::days::Part;
use crate::util::read_lines;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

const MAPS: [u8; 4] = [0b0001, 0b0010, 0b0100, 0b1000];

pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut map = Vec::new();

  for l in lines {
    let line = l.expect("Failed to read line");
    let mut row = Vec::new();
    for ch in line.chars() {
      row.push(ch);
    }
    map.push(row);
  }

  match part {
    Part::P1 => Ok(part_1(map, false)),
    Part::P2 => Ok(part_1(map, true)),
  }
}


fn part_1(map: Vec<Vec<char>>, discount: bool) -> u64 {
  let mut visited = Vec::new();
  let mut perims = Vec::new();
  let mut area_indexes = Vec::new();
  let mut areas = Vec::new();
  let mut perim = Vec::new();

  for _y in 0..map.len() {
    let mut vs = Vec::new();
    let mut ps = Vec::new();
    let mut ai = Vec::new();
    for _x in 0..map[0].len() {
      vs.push(false);
      ps.push(0u8);
      ai.push(0usize);
    }
    visited.push(vs);
    perims.push(ps);
    area_indexes.push(ai);
  }

  for y in 0..map.len() {
    for x in 0..map[0].len() {
      if visited[y][x] {
        continue;
      }

      let (area, p) = determine_area(
        &map, x, y, &mut visited, 
        &mut perims, &mut area_indexes,
        areas.len(), discount
      );

      areas.push(area);
      perim.push(p);
    }
  }

  println!("{:?}", areas);
  println!("{:?}", perim);

  let mut result = 0;

  for i in 0..areas.len() {
    result += areas[i] as u64 * perim[i] as u64;
  }

  result
}

fn determine_area(
  map: &Vec<Vec<char>>, 
  x: usize, 
  y: usize,
  visited: &mut Vec<Vec<bool>>,
  perims: &mut Vec<Vec<u8>>,
  area_indexes: &mut Vec<Vec<usize>>,
  area_index: usize,
  discount: bool
) -> (u32, u32) {
  const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
  let mut checks = BTreeMap::new();
  checks.insert((0, 1), vec![(1, 0), (-1, 0)]);
  checks.insert((0, -1), vec![(1, 0), (-1, 0)]);
  checks.insert((1, 0), vec![(0, 1), (0, -1)]);
  checks.insert((-1, 0), vec![(0, 1), (0, -1)]);

  let mut to_visit = vec![(y, x)];
  let mut area = 0;
  let mut perim = 0;

  let height = map.len();
  let width = map[0].len();
  let plant_type = map[y][x];
  let mut edges = BTreeMap::new();

  loop {
    let Some((y, x)) = to_visit.pop() else {
      break;
    };

    if visited[y][x] {
      continue;
    }
    visited[y][x] = true;
    area_indexes[y][x] = area_index;
    area += 1;

    for i in 0..DIRS.len() {
      let (dy, dx) = DIRS[i];

      let nx = x as i32 + dx;
      let ny = y as i32 + dy;

      let is_edge = if 
        nx < 0 || nx >= width as i32 ||
        ny < 0 || ny >= height as i32 {
        true   
      } else {
        let nx = nx as usize;
        let ny = ny as usize;

        if map[ny][nx] == plant_type {
          to_visit.push((ny, nx));
          false
        } else {
          true
        }
      };

      if !is_edge {
        continue;
      }

      if !edges.contains_key(&DIRS[i]) {
        edges.insert(DIRS[i], BTreeSet::new());
      }
      edges.get_mut(&DIRS[i]).unwrap().insert((y, x));

      perims[y][x] |= MAPS[i];
      perim += 1;
    }
  }

  if !discount {
    return (area, perim);
  }

  perim = 0;
  // println!("{:?}", edges);

  for (check, mut ps) in edges {
    // println!("{:?}", ps);
    loop {
      let Some((mut y, mut x)) = ps.pop_last() else {
        break;
      };

      perim += 1;
      // println!("{} {}", y, x);

      for (dy, dx) in checks.get(&check).unwrap() {
        loop {
          let ny = y as i32 - *dy as i32;
          let nx = x as i32 - *dx as i32;

          if ny < 0 || nx < 0 {
            break;
          }

          y = ny as usize;
          x = nx as usize;
          // println!(" - {} {}", y, x);

          if !ps.remove(&(y, x)) {
            break;
          }
        }
      }

    }
  }

  (area, perim)
}
