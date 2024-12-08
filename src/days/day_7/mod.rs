use crate::days::Part;
use crate::util::read_lines;

#[derive(Debug)]
struct Equation {
   target: u64,
   parts: Vec<u64>,
}

#[derive(Debug)]
enum Opp {
   Add,
   Mul,
   Cat,
}


pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut input = Vec::new();

  for line in lines {
    let l = line.expect("Input");
    let (target, parts) = l.split_once(":").expect("Failed to split once");

    let parts = parts.trim().split(" ").into_iter()
      .map(|p| p.parse::<u64>().expect("Failed to parse target"))
      .collect();

    input.push(Equation {
      target: target.parse::<u64>().expect("Failed to parse target"),
      parts,
    });
  }

  match part {
    Part::P1 => Ok(part_1(input)),
    Part::P2 => Ok(part_2(input)),
  }
}

fn part_1(input: Vec<Equation>) -> u64 {
   let mut sum = 0;

   for eq in input {
      if can_solve(&eq, false) {
         sum += eq.target;
      }
   }
   
   sum
}

fn can_solve(eq: &Equation, use_cat: bool) -> bool {
   const OPPS: [Opp; 2] = [Opp::Add, Opp::Mul];
   const OPPS_CAT: [Opp; 3] = [Opp::Add, Opp::Mul, Opp::Cat];

   let mut prevs = vec![eq.parts[0]];

   for n in eq.parts.iter().skip(1) {
      let mut new_prevs = Vec::new();

      for prev in &prevs {
         let iter = if use_cat { OPPS_CAT.iter() } else { OPPS.iter() };

         for opp in iter {
            let res = match opp {
               Opp::Add => Some(prev + n),
               Opp::Mul => Some(prev * n),
               Opp::Cat => {
                  10u64.checked_pow(n.ilog10() + 1)
                     .map(|pow| prev.checked_mul(pow))
                     .flatten()
                     .map(|pr| pr.checked_add(*n))
                     .flatten()
               }
            };

            let Some(r) = res else {
               continue;
            };

            if r > eq.target {
               continue;
            }
            new_prevs.push(r);
         }
      }

      prevs = new_prevs;
   }

   for result in prevs {
      if result == eq.target {
         return true;
      }
   }
   return false;
}

fn part_2(input: Vec<Equation>) -> u64 {
   let mut sum = 0;

   for eq in input {
      if can_solve(&eq, true) {
         sum += eq.target;
      }
   }
   
   sum
}