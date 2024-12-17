
use crate::days::Part;
use crate::util::read_lines;


pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut iter = lines.into_iter();

  let a = parse_reg(&mut iter);
  let b = parse_reg(&mut iter);
  let c = parse_reg(&mut iter);

  let _blank = iter.next();

  let line = iter.next().unwrap().expect("Failed to read opps line");
  let (_prefix, opps) = line.split_once(": ").unwrap();

  let opps = opps.split(",")
      .map(|n| n.parse::<u8>().expect("Failed to parse opp"))
      .collect();
  

  match part {
    Part::P1 => Ok(part_1((a, b, c), &opps, &Vec::new(), false)),
    Part::P2 => Ok(part_2((a, b, c), opps)),
  }
}

fn part_1(reg: (u64, u64, u64), opps: &Vec<u8>, res: &Vec<u8>, fix: bool) -> u64 {
  let mut ca = reg.0;
  let mut stdout = Vec::new();

  loop {
    let (_, mut rb, mut rc) = reg.clone();
    let mut pc: usize = 0;
    let mut ra = ca;


    stdout.clear();

    loop {
      if pc >= opps.len() - 1 {
        break;
      }

      let opp = opps[pc];

      let literal = opps[pc + 1] as u64;
      let combo = match opps[pc + 1] {
        n @ 0..=3 => n as u64,
        4 => ra,
        5 => rb,
        6 => rc,
        7 => u64::MAX,
        _ => panic!("Invalid opp code"),
      };

      pc += 2;

      match opp {
        0 => ra = ra / 2u64.pow(combo as u32),
        1 => rb = rb ^ literal,
        2 => rb = combo % 8,
        3 => if ra != 0 { pc = literal as usize; },
        4 => rb = rb ^ rc,
        5 => {
          stdout.push(combo % 8);
          if fix && stdout[stdout.len() - 1]  != res[stdout.len() - 1] as u64 {
            break;
          }
        },
        6 => rb = ra / 2u64.pow(combo as u32),
        7 => rc = ra / 2u64.pow(combo as u32),
        _ => panic!("Invalid opp code"),
      }
    }

    if !fix {
      break;
    }
    let mut solved = stdout.len() == res.len();
    if solved {
      for i in 0..stdout.len() {
        if stdout[i] != res[i] as u64 {
          solved = false;
          break;
        }
      }
    }

    if solved {
      break;
    } else {
      ca += 1;
      if ca % 1_000_000 == 0 {
        println!("{} - {}", ca, stdout.len());
      }
    }
  }

  print!("output: ");
  for n in stdout {
    print!("{},", n)
  }
  println!();

  ca
}

fn part_2(reg: (u64, u64, u64), opps: Vec<u8>) -> u64 {
  let mut req = Vec::new();
  let mut opp_i = opps.len() - 1;
  let mut ra = 0;

  loop {
    if opp_i == 0 {
      break;
    }
    req.insert(0, opps[opp_i]);
    let tmp = part_1((ra, reg.1, reg.2), &opps, &req, true);
    ra = tmp * 8;
    opp_i -= 1;
  }

  part_1((ra, reg.1, reg.2), &opps, &opps, true)
}

fn parse_reg(lines: &mut dyn std::iter::Iterator<Item = std::io::Result<String>>) -> u64 {
  let line = lines.next().unwrap().expect("Failed to read reg line");
  let (_prefix, value) = line.split_once(": ").unwrap();
  value.parse::<u64>().expect("Failed to parse reg")
}