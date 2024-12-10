
use crate::days::Part;
use crate::util::read_lines;

use std::collections::BTreeSet;


pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut input = Vec::new();
  let mut starts = Vec::new();

  for l in lines {
    let line = l.expect("Failed to read line");
    let mut row = Vec::new();
    for ch in line.chars() {
      let n = match ch {
        '.' => 20,
        ch => ch as u8 - '0' as u8,
      };
      if n == 0 {
        starts.push((input.len(), row.len()));
      }
      row.push(n);
    }
    input.push(row);
  }

  match part {
    Part::P1 => Ok(part_1(input, starts)),
    Part::P2 => Ok(part_2(input, starts)),
  }
}

fn part_1(input: Vec<Vec<u8>>, starts: Vec<(usize, usize)>) -> u64 {
  let mut count = 0;

  for start in starts {
    count += dijkstra_distinct(&input, false, start.0, start.1);
  }

  count
}

fn part_2(input: Vec<Vec<u8>>, starts: Vec<(usize, usize)>) -> u64 {
  let mut count = 0;

  for start in starts {
    count += dijkstra_distinct(&input, true, start.0, start.1);
  }

  count
}


fn dijkstra_distinct(graph: &Vec<Vec<u8>>, is_p2: bool, y: usize, x: usize) -> u64 {
  const GOAL: u8 = 9;
  const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

  let mut visited = Vec::new();
  let mut distances = Vec::new();
  let mut unvisited = Vec::new();
  let mut paths_to = Vec::new();
  let mut goals = Vec::new();
  let mut result = 0;

  for row in graph {
    let mut v = Vec::new();
    let mut p = Vec::new();
    let mut dist = Vec::new();
    for _e in row {
      v.push(false);
      p.push(BTreeSet::new());
      dist.push(u32::MAX);
    }
    visited.push(v);
    distances.push(dist);
    paths_to.push(p);
  }

  let width = visited[0].len() as i32;
  let height = visited.len() as i32;

  distances[y][x] = 0;
  unvisited.push((y, x));

  loop {
    let Some((y, x)) = unvisited.pop() else {
      break;
    };

    if visited[y][x] {
      continue;
    }
    visited[y][x] = true;

    if graph[y][x] == GOAL {
      result += 1;
      goals.push((y, x));
      continue;
    }

    let mut changed = false;

    for (dy, dx) in DIRS {
      let nx = x as i32 + dx;
      let ny = y as i32 + dy;

      if nx < 0 || nx >= width ||
         ny < 0 || ny >= height {
          continue;
      }

      let nx = nx as usize;
      let ny = ny as usize;

      if graph[ny][nx] <= graph[y][x] ||
         graph[ny][nx] - graph[y][x] != 1 {
        continue; // Only move to space which is one heigher
      }

      paths_to[ny][nx].insert((y, x));

      if visited[ny][nx] {
        continue;
      }

      changed = true;
      unvisited.push((ny, nx));
      distances[ny][nx] = std::cmp::min(
        distances[ny][nx], distances[y][x] + 1
      );
    }

    if !changed {
      continue;
    }
   
    unvisited.sort_by(|a, b| {
      distances[b.0][b.1].cmp(&distances[a.0][a.1])
    });
    // println!("dist: {:?}", distances);
    // println!("vist: {:?}", unvisited);
  }
  // println!("res: {:?}", result);

  if !is_p2 {
    return result;
  }

  let mut paths = 0;

  for (y, x) in goals {
    let mut visit = Vec::new();

    visit.push((y, x));

    loop {
      let Some((y, x)) = visit.pop() else {
        break;
      };

      if graph[y][x] == 0 {
        paths += 1;
        continue;
      }

      for p in &paths_to[y][x] {
        visit.push(*p);
      }
    }
  }

  paths
}