
use crate::days::Part;
use crate::util::read_lines;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut pairs = Vec::new();

  for l in lines {
    let line = l.expect("Failed to read line");
    let (a, b) = line.split_once("-").unwrap();
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    pairs.push(([ab[0], ab[1]], [bb[0], bb[1]]));
  }

  match part {
    Part::P1 => Ok(part_1(pairs)),
    Part::P2 => Ok(part_2(pairs)),
  }
}

fn part_2(pairs: Vec<([u8;2], [u8;2])>) -> u64 {
  let mut single_conns = BTreeMap::new();
  let mut max = BTreeSet::new();

  for (a, b) in pairs {
    add_or_create(&mut single_conns, &a, b);
    add_or_create(&mut single_conns, &b, a);

    let b_set = single_conns.get(&b).unwrap();

    if b_set.len() < max.len() {
      // Can't be larger than our biggest intersection
      continue;
    }

    // println!("testing: {} {}", ts(&a), ts(&b));
    let mut curr = BTreeSet::new();

    curr.insert(a);
    curr.insert(b);

    loop {
      let mut contender = curr.clone();
      let mut updated = false;
      for element in single_conns.get(curr.first().unwrap()).unwrap() {
        if contender.contains(element) {
          continue;
        }
        // println!("possible: {}", ts(element));
        let mut valid = true;
        for other in curr.iter().skip(1) {
          if !single_conns.get(other).unwrap().contains(element) {
            valid = false;
            break;
          }
        }
        if valid {
          contender.insert(*element);
          updated = true;
          break;
        }
      }

      if !updated {
        break;
      }
      
      curr = contender;

      if curr.len() > max.len() {
        print!("max:");
        for c in &curr {
          print!(" {}", ts(c));
        }
        println!();
        max = curr.clone();
      } 
    }
  }

  print!("password: ");
  for (i, ell) in max.iter().enumerate() {
    if i == 0 {
      print!("{}", ts(&ell));
    } else {
      print!(",{}", ts(&ell));
    }
  }
  println!();

  0
}

fn part_1(pairs: Vec<([u8;2], [u8;2])>) -> u64 {

  let mut single_conns = BTreeMap::new();
  let mut triplets = BTreeSet::new();

  for (a, b) in pairs {
    add_or_create(&mut single_conns, &a, b);
    add_or_create(&mut single_conns, &b, a);

    let b_set = single_conns.get(&b).unwrap();
    for c in single_conns.get(&a).unwrap() {
      if b_set.contains(c) {
        triplets.insert((a, b, *c));
      }
    }
  }

  let mut count = 0;
  let id = 't' as u8;

  for (a, b, c) in triplets {
    if a[0] == id || b[0] == id || c[0] == id {
      count += 1;
    }
  }

  count
}

fn add_or_create(pairs: &mut BTreeMap<[u8;2], BTreeSet<[u8;2]>>, a: &[u8;2], b: [u8;2]) {
  if let Some(set) = pairs.get_mut(a) {
    set.insert(b);
  } else {
    let mut set = BTreeSet::new();
    set.insert(b);
    pairs.insert(a.clone(), set);
  }
}

fn ts<'a>(bytes: &'a [u8;2]) -> &'a str {
  std::str::from_utf8(bytes).expect("Failed to parse as str")
}
