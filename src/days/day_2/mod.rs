use crate::days::Part;
use crate::util::read_lines;


pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
   let Ok(lines) = read_lines(file_name) else {
      return Err("Failed to read lines");
   };

   let reports: Vec<Vec<u32>> = lines.into_iter()
      .map(|line| {
         let string = line.expect("unwable to read line");
         
         string.split(" ")
            .into_iter()
            .map(|num| num.parse::<u32>().expect("failed to parse num"))
            .collect::<Vec<u32>>()
   }).collect();

   match part {
      Part::P1 => Ok(part_1(reports)),
      Part::P2 => Ok(part_2(reports)),
   }
}


fn part_1(reports: Vec<Vec<u32>>) -> u64 {
  let mut safe_count = 0;
  for report in reports {
    let increasing = report[0] < report[1];
    let mut safe = true;

    for i in 1..report.len() {
      let prev = report[i - 1];
      let next = report[i];

      if (prev < next) != increasing {
        safe = false;
        break;
      }

      let diff = prev.abs_diff(next);

      if diff < 1 || diff > 3 {
        safe = false;
        break;
      }
    }

    if safe {
      safe_count += 1;
    }
  }

  safe_count
}

fn part_2(reports: Vec<Vec<u32>>) -> u64 {
  let mut safe_count = 0;
  for report in reports {
    for skip in -1..(report.len() as i64) {
      let start = if skip == 0 {
        1
      } else {
        0
      };

      let increasing = report[start] < report[start +  if skip == 1 {2} else {1}];
      let mut safe = true;

      for i in (start+1)..report.len() {
        if i as i64 == skip {
          continue;
        }
        let prev_i = if (i - 1) as i64 == skip {
          i - 2
        } else {
          i - 1
        };

        let prev = report[prev_i];
        let next = report[i];

        if (prev < next) != increasing {
          safe = false;
          break;
        }

        let diff = prev.abs_diff(next);

        if diff < 1 || diff > 3 {
          safe = false;
          break;
        }
      }

      if safe {
        safe_count += 1;
        break;
      }
    }
  }

  safe_count
}