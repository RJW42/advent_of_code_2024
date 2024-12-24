

use crate::days::Part;
use crate::util::read_lines;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

type Reg = [u8; 3];

#[derive(Debug)]
struct Gate {
  in_0: Reg,
  in_1: Reg,
  out: Reg,
  gate_type: GateType,
}

#[derive(Debug, Eq, PartialEq)]
enum GateType {
  And,
  Or,
  XOr
}

pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut gates = Vec::new();
  let mut regs = BTreeMap::new();

  let mut parsing_reg = true;
  for l in lines {
    let line = l.expect("Failed to read line");
    if line.len() == 0 {
      parsing_reg = false;
      continue;
    }
    if parsing_reg {
      let (reg, val) = line.split_once(": ").unwrap();
      let bs = reg.as_bytes();
      let val = val.parse::<u8>().expect("Failed to parse reg") == 1;
      regs.insert([bs[0], bs[1], bs[2]], val);
    } else {
      let (reg_in_0, extra) = line.split_once(" ").unwrap();
      let (type_s, extra2) = extra.split_once(" ").unwrap();
      let (reg_in_1, reg_out) = extra2.split_once(" -> ").unwrap();
      let gate_type = match type_s {
        "AND" => GateType::And,
        "OR" => GateType::Or,
        "XOR" => GateType::XOr,
        _ => panic!("Invalid gate type"),
      };
      let i0 = reg_in_0.as_bytes();
      let i1 = reg_in_1.as_bytes();
      let o = reg_out.as_bytes();
      gates.push(Gate {
        in_0: [i0[0], i0[1], i0[2]],
        in_1: [i1[0], i1[1], i1[2]],
        out: [o[0], o[1], o[2]],
        gate_type
      });
    }
  }

  match part {
    Part::P1 => Ok(part_1(gates, regs)),
    Part::P2 => Ok(part_2(gates, regs)),
  }
}

fn part_1(gates: Vec<Gate>, regs: BTreeMap<Reg, bool>) -> u64 {
  let mut unused_regs = regs.clone();
  let mut all_regs = regs;
  let mut fired = vec![false; gates.len()];

  loop {
    let mut changed = false;

    for i in 0..gates.len() {
      if fired[i] {
        continue;
      }
      let gate = &gates[i];

      let Some(a) = all_regs.get(&gate.in_0) else {
        continue;
      };
      let Some(b) = all_regs.get(&gate.in_1) else {
        continue;
      };

      let result = match gate.gate_type {
        GateType::And => a & b,
        GateType::Or  => a | b, 
        GateType::XOr => a ^ b,
      };

      unused_regs.remove(&gate.in_0);
      unused_regs.remove(&gate.in_1);
      unused_regs.insert(gate.out.clone(), result);
      all_regs.insert(gate.out.clone(), result);
      fired[i] = true;
      changed = true;
    }

    if !changed {
      break;
    }
  }

  let mut sum = 0;
  // println!("final");
  for (_reg, val) in unused_regs.iter().rev() {
    // println!(" - {} : {}", ts(&reg), val);
    sum = (sum << 1) + *val as u64;
  }

  sum
}

fn part_2(gates: Vec<Gate>, _regs: BTreeMap<Reg, bool>) -> u64 {
  let mut largest_z = gates[0].out.clone();
  for g in &gates {
    if largest_z[0] != 'z' as u8 || ( 
       g.out[0] == 'z' as u8 && (
       g.out[1] > largest_z[1] || (
        g.out[1] == largest_z[1] && g.out[2] > largest_z[2]
      ))
    ) {
      largest_z = g.out.clone();
    }
  }

  let mut wrong = BTreeSet::new();
  for i in 0..gates.len() {
    let g = &gates[i];
    if g.out[0] == 'z' as u8 && 
       g.gate_type != GateType::XOr &&  
       g.out != largest_z {
      wrong.insert(g.out.clone());
    }
    if g.gate_type == GateType::XOr && 
       !['x', 'y', 'z'].contains(&(g.out[0] as char)) && 
       !['x', 'y', 'z'].contains(&(g.in_0[0] as char)) && 
       !['x', 'y', 'z'].contains(&(g.in_1[0] as char)) {
      wrong.insert(g.out.clone());
    }
    if g.gate_type == GateType::XOr {
      for j in 0..gates.len() {
        let g2 = &gates[j];
        if g2.gate_type == GateType::Or && (
          g2.in_0 == g.out || g2.in_1 == g.out
        )  {
          wrong.insert(g.out.clone());
        }
      }
    }
    if g.gate_type == GateType::And &&
       g.in_0 != ['x' as u8, '0' as u8, '0' as u8] &&
       g.in_1 != ['x' as u8, '0' as u8, '0' as u8] {
      for j in 0..gates.len() {
        let g2 = &gates[j];
        if g2.gate_type != GateType::Or && (
          g2.in_0 == g.out || g2.in_1 == g.out
        )  {
          wrong.insert(g.out.clone());
        }
      }
    }
  }

  print!("final: {}", ts(&wrong.first().unwrap()));
  for r in wrong.iter().skip(1) {
    print!(",{}", ts(r));
  }
  println!();

  0
}

fn ts<'a>(bytes: &'a Reg) -> &'a str {
  std::str::from_utf8(bytes).expect("Failed to parse as str")
}