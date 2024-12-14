
use crate::days::Part;
use crate::util::read_lines;


#[derive(Debug, Clone, Copy)]
struct Robot {
  pos: (u16, u16),
  vel: (i16, i16),
}


pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut input = Vec::new();

  for l in lines {
    let line = l.expect("Unable to read line");
    let (fps, fvs) = line.split_once(" ").unwrap();
    let (_, ps) = fps.split_once("=").unwrap();
    let (_, vs) = fvs.split_once("=").unwrap();
    let (pxs, pys) = ps.split_once(",").unwrap();
    let (vxs, vys) = vs.split_once(",").unwrap();

    let px = pxs.parse::<u16>().expect("Failed to parse x");
    let py = pys.parse::<u16>().expect("Failed to parse y");
    let vx = vxs.parse::<i16>().expect("Failed to parse vx");
    let vy = vys.parse::<i16>().expect("Failed to parse vy");

    input.push(Robot {
      pos: (py, px),
      vel: (vy, vx),
    });
  }

  let w = 101;
  let h = 103;

  match part {
    Part::P1 => Ok(part_1(&mut input, w, h, 100)),
    Part::P2 => Ok(part_2(input, w, h)),
  }
}


fn part_1(robots: &mut Vec<Robot>, width: u16, height: u16, steps: u16) -> u64 {
  // println!("{:?}", robots);
  advance_robots(robots, width, height, steps);

  // println!("{:?}", robots);

  let mut quads: [u16; 4] = [0, 0, 0, 0];

  for robot in robots.iter() {
    let xq = if robot.pos.1 > width / 2 {1} else {0};
    let yq = if robot.pos.0 > height / 2 {1} else {0};
    let index = (xq << 1) | yq;

    quads[index as usize] += (robot.pos.1 != width / 2 && robot.pos.0 != height / 2) as u16;
  }

  // println!("{:?}", quads);
  let mut total = 1;

  for i in 0..4 {
    total *= quads[i] as u64;
  }

  total
}

fn advance_robots(robots: &mut Vec<Robot>, width: u16, height: u16, steps: u16) {
  for robot in robots.iter_mut() {
    let dx = (robot.pos.1 as i16 + robot.vel.1 * (steps as i16 % width as i16)) % width as i16;
    let dy = (robot.pos.0 as i16 + robot.vel.0 * (steps as i16 % height as i16 )) % height as i16;

    let dx = if dx < 0 {width as i16 + dx} else {dx};
    let dy = if dy < 0 {height as i16 + dy} else {dy};

    robot.pos.1 = dx as u16;
    robot.pos.0 = dy as u16;
  }
}

fn part_2(mut robots: Vec<Robot>, width: u16, height: u16) -> u64 {
  let initial = robots.clone();

  for step in 0..(width * height + 1) {
    robots = initial.clone();
    advance_robots(&mut robots, width, height, step);
    if print_robots(&robots, width, height) {
      return step as u64
    };
    
    let mut quads: [u16; 4] = [0, 0, 0, 0];

    for robot in robots.iter() {
      let xq = if robot.pos.1 > width / 2 {1} else {0};
      let yq = if robot.pos.0 > height / 2 {1} else {0};
      let index = (xq << 1) | yq;

      quads[index as usize] += (robot.pos.1 != width / 2 && robot.pos.0 != height / 2) as u16;
    }
  }

  panic!("Could not find yo tree")
}

fn print_robots(robots: &Vec<Robot>, width: u16, height: u16) -> bool {
  let mut pixels = vec![false; (height * width) as usize];
  let mut visited = vec![false; (height * width) as usize];
  let index = |x, y| {(x * height + y) as usize};

  for robot in robots {
    pixels[(robot.pos.1 * height + robot.pos.0) as usize] = true;
  }

  let mut max_line = 0;

  for y in 0..height {
    for x in 0..width {
      if visited[index(x, y)] || !pixels[index(x, y)] {
        continue;
      }
      visited[index(x, y)] = true;
      let mut curr_line = 1;

      for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let mut cx = x;
        let mut cy = y;

        loop {
          let nx = cx as i16 + dx;
          let ny = cy as i16 + dy;

          if nx < 0 || ny < 0 || 
             nx as u16 >= width || ny as u16 >= height {
            break;
          }

          cx = nx as u16;
          cy = ny as u16;

          if visited[index(cx, cy)] || !pixels[index(cx, cy)] {
            break;
          }
          visited[index(cx, cy)] = true;
          curr_line += 1;
        }
      }

      max_line = u16::max(max_line, curr_line);
    }
  }

  if max_line < 10 {
    return false;
  }

  for y in 0..height {
    for x in 0..width {
      print!("{}", if pixels[index(x, y)] {'#'} else {'.'});
    }
    println!();
  }
  return true;
}
