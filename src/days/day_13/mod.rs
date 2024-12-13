

use crate::days::Part;
use crate::util::read_lines;

type Equations = [[f64; 3]; 2];

pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut input = Vec::new();

  let mut iter = lines.into_iter();
  loop {
    let Some(l1) = iter.next() else {
      break;
    };
    let l2 = iter.next().unwrap();
    let l3 = iter.next().unwrap();
    let _blank_line = iter.next();

    let (ax, ay) = parse_button_xy(l1.expect("No l1"));
    let (bx, by) = parse_button_xy(l2.expect("No l2"));
    let (xs, ys) = parse_solution_xy(l3.expect("No l3"));

    let e1 = [ax as f64, bx as f64, xs as f64];
    let e2 = [ay as f64, by as f64, ys as f64];

    input.push([e1, e2]);
  }


  match part {
    Part::P1 => Ok(part_1(input)),
    Part::P2 => Ok(part_2(input)),
  }
}

fn parse_button_xy(line: String) -> (u64, u64) {
  let (_prefix, xy) = line.split_once(": ").unwrap();
  let (x, y) = xy.split_once(", ").unwrap();

  (
    x.split_once("+").unwrap().1.parse::<u64>().expect("Failed to parse x"),
    y.split_once("+").unwrap().1.parse::<u64>().expect("Failed to parse y")
  )
}

fn parse_solution_xy(line: String) -> (u64, u64) {
  let (_prefix, xy) = line.split_once(": ").unwrap();
  let (x, y) = xy.split_once(", ").unwrap();

  (
    x.split_once("=").unwrap().1.parse::<u64>().expect("Failed to parse x"),
    y.split_once("=").unwrap().1.parse::<u64>().expect("Failed to parse y")
  )
}

fn part_1(eqs: Vec<Equations>) -> u64 {
  let mut solution = 0;

  for mut eq in eqs {
    let orig = eq.clone();

    // Reduce first element of second equation
    make_leading_one(&mut eq[0]);
    let r1 = make_reduction_eq(&eq, 0, 1);
    subtract(&mut eq, &r1, 1);
    assert_non_zero(eq[1]);

    // Reduce second element of first equation
    make_leading_one(&mut eq[1]);
    let r2 = make_reduction_eq(&eq, 1, 0);
    subtract(&mut eq, &r2, 0);
    assert_non_zero(eq[0]);

    let na = eq[0][2].round() as u64;
    let nb = eq[1][2].round() as u64;

    let good = {
      orig[0][0] as u64 * na + orig[0][1] as u64 * nb == orig[0][2] as u64 &&
      orig[1][0] as u64 * na + orig[1][1] as u64 * nb == orig[1][2] as u64
    };

    if !good {
      println!(" - na: {:?}", eq);
      continue;
    }

    println!(" - ok: {:?}", eq);
    solution += (na * 3 + nb) as u64;
  }

  solution
}

fn subtract(eqs: &mut Equations, eq: &[f64; 3], i: usize) {
  for j in 0..3 {
    eqs[i][j] -= eq[j]
  }
}

fn make_reduction_eq(eqs: &Equations, a: usize, b: usize) -> [f64; 3] {
  let reducing = if eqs[a][0] == 0.0 {1} else {0};
  if eqs[a][reducing] != 1.0 {
    panic!("Leading number should be one for a");
  }

  let mul = eqs[b][reducing];
  if mul == 0.0 {
    panic!("can't multiple by zero");
  }

  [eqs[a][0] * mul, eqs[a][1] * mul, eqs[a][2] * mul]
}

fn make_leading_one(eq: &mut [f64; 3]) {
  let index = if eq[0] == 0.0 {1} else {0};
  let divisor = eq[index];

  for i in 0..3 {
    eq[i] /= divisor;
  }
}

fn assert_non_zero(eq: [f64; 3]) {
  for i in 0..3 {
    if eq[i] != 0.0 {
      return;
    }
  }
  panic!("all zero");
}

fn part_2(mut eqs: Vec<Equations>) -> u64 {
  for eq in &mut eqs {
    eq[0][2] += 10000000000000f64;
    eq[1][2] += 10000000000000f64;
  }

  part_1(eqs)
}