use crate::days::Part;
use crate::util::read_lines;


pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut input = Vec::new();

  for line in lines {
    let l = line.expect("Input");
    input.push(l.chars().collect());
  }

  println!("{:?}", input);


  match part {
    Part::P1 => Ok(part_1(input)),
    Part::P2 => Ok(part_2(input)),
  }
}

fn part_1(input: Vec<Vec<char>>) -> u64 {
  let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
  let height = input.len();
  let width = input[0].len();

  let mut xmas_count = 0;

  for y in 0..height {
    for x in 0..width {
      if input[y][x] != 'X' {
        continue;
      }
      // println!("from {} {}", x, y);

      for (dx, dy) in &directions {
        let mut cx = x;
        let mut cy = y;

        let mut expected = 'M';
        // println!(" - {} {}", dx, dy);

        loop {
          let nx = cx as i64 + dx;
          let ny = cy as i64 + dy;

          if nx < 0 || nx as usize >= width || ny < 0 || ny as usize >= height {
            break;
          }

          cx = nx as usize;
          cy = ny as usize;

          // println!("    - {} {} {}", cx, cy, input[cy][cx]);

          if input[cy][cx] != expected {
            break;
          }

          if expected == 'S' {
            // println!("found {} {}", x, y);
            xmas_count += 1;
            break;
          } 

          expected = match expected {
            'M' => 'A',
            'A' => 'S',
            _ => todo!(),
          };
        }
      }
    } 
  }

  xmas_count
}


fn part_2(input: Vec<Vec<char>>) -> u64 {
  let directions = vec![((-1, -1), (1, 1)), ((1, 1), (-1, -1)), ((-1, 1), (1, -1)), ((1, -1), (-1, 1))];
  let height = input.len();
  let width = input[0].len();

  let mut xmas_count = 0;

  for y in 0..height {
    for x in 0..width {
      if input[y][x] != 'A' {
        continue;
      }
      // println!("from {} {}", x, y);

      let mut count = 0;

      for ((s_dx, s_dy), (m_dx, m_dy)) in &directions {
        let s_nx = x as i64 + s_dx;
        let s_ny = y as i64 + s_dy;
        let m_nx = x as i64 + m_dx;
        let m_ny = y as i64 + m_dy;
        if s_nx < 0 || s_nx as usize >= width 
        || s_ny < 0 || s_ny as usize >= height 
        || m_nx < 0 || m_nx as usize >= width 
        || m_ny < 0 || m_ny as usize >= height {
          continue;
        }

        let s_cx = s_nx as usize;
        let s_cy = s_ny as usize;
        let m_cx = m_nx as usize;
        let m_cy = m_ny as usize;
        // println!(" - s {} {} {}, m {} {} {}", s_cx, s_cy, input[s_cy][s_cx], m_cx, m_cy, input[m_cy][m_cx]);

        if input[s_cy][s_cx] != 'S' || input[m_cy][m_cx] != 'M' {
          continue;
        }

        count += 1;
      }

      if count == 2 {
        xmas_count += 1;
      } else if count > 2 {
        todo!()
      }
    } 
  }

  xmas_count
}