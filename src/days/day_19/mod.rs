
use crate::days::Part;
use crate::util::read_lines;

use std::collections::BTreeMap;

#[derive(Debug)]
enum Node {
  Child(usize),
  Leaf(Vec<char>),
}

pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  let Ok(lines) = read_lines(file_name) else {
    return Err("Failed to read lines");
  };

  let mut towels = Vec::new();
  let mut patterns = Vec::new();

  let mut iter = lines.into_iter();

  iter.next()
    .unwrap().expect("Failed to read pattern line")
    .split(", ")
    .for_each(|p| towels.push(p.to_owned()));

  let _blank_line = iter.next();

  for l in iter {
    let line = l.expect("Failed to read line");
    patterns.push(line);
  }


  match part {
    Part::P1 => Ok(part_1(towels, patterns)),
    Part::P2 => Ok(part_2(towels, patterns)),
  }
}

fn part_1(towels: Vec<String>, patterns: Vec<String>) -> u64 {
  let mut possible = 0;
  let tree = make_parse_tree(&towels);

  println!("{:?}", tree);
  // println!("{:?}", tree[15].keys());
  // println!("{:?}", tree[29].keys());
  
  for t in towels {
    if matches(&tree, t.as_str()) == 0 {
      panic!("Should be able to match {}", t)
    }

  }

  for p in patterns {
    if matches(&tree, p.as_str()) > 0 {
      possible += 1;
    }
  }

  possible
}

fn matches(tree: &Vec<BTreeMap<char, Vec<Node>>>, p: &str) -> u64 {
    let mut queue = Vec::new();
    let mut visited = BTreeMap::new();
    let chars: Vec<_> = p.chars().collect();

    if !tree[0].contains_key(&chars[0]) {
      return 0;
    }

    queue.push(((0, 0, 0), Vec::<(usize, usize, usize)>::new()));
    println!("checking: {}", p);

    loop {
      let Some((element, prevs)) = queue.pop() else {
        break;
      };
      if visited.contains_key(&element) {
        let inc = *visited.get(&element).unwrap();
        // println!("hmmm {:?}, {}", element, inc);
        for prev in &prevs {
          if visited.contains_key(prev) {
            *visited.get_mut(prev).unwrap() += inc;
          } else {
            visited.insert(prev.clone(), inc);
          }
        }
        continue;
      }

      let mut new_prevs = prevs.clone();
      new_prevs.push(element.clone());

      let (pi, ni, ti) = element;
      
      let curr = &tree[ti].get(&chars[pi]).unwrap();
      if ni + 1 < curr.len() {
        // More children to check in future
        queue.push(((pi, ni + 1, ti), new_prevs.clone()));
      }

      match &curr[ni] {
        Node::Leaf(leaf) => {
          let mut matched = true;
          for i in 0..leaf.len() {
            if pi + i + 1 >= chars.len() || 
              chars[pi + i + 1] != leaf[i] {
              matched = false;
              break;
            }
          }
          if matched {
            if pi + leaf.len() + 1 == chars.len() {
              // Matched and reached end
              // println!("good: {}", p);
              for prev in new_prevs {
                // println!("  - {:?} {:?}", prev, visited.get(&prev));
                if visited.contains_key(&prev) {
                  *visited.get_mut(&prev).unwrap() += 1;
                } else {
                  visited.insert(prev, 1);
                }
              }
            } else if tree[0].contains_key(&chars[pi + leaf.len() + 1]) {
              // Matched, not reached end
              queue.push(((pi + leaf.len() + 1, 0, 0), new_prevs));
            }
          } else {
            // Not solvavble, mark
            visited.insert(new_prevs[new_prevs.len() - 1], 0);
          }
        },
        Node::Child(ci) => {
          if pi + 1 < chars.len() && tree[*ci].contains_key(&chars[pi + 1]) {
            // Child matches current char, check it
            queue.push(((pi + 1, 0, *ci), new_prevs));
          }
        }
      }
    }
    *visited.get(&(0, 0, 0)).unwrap_or(&0)
}

fn make_parse_tree(towels: &Vec<String>) -> Vec<BTreeMap<char, Vec<Node>>> {
  let mut output: Vec<BTreeMap<char, Vec<Node>>> = Vec::new();

  output.push(BTreeMap::new());

  for towel in towels {
    // println!("{:?}", output);
    let chars: Vec<_> = towel.chars().collect();
    let mut ci = 0;
    'outer: for i in 0..chars.len() {
      if !output[ci].contains_key(&chars[i]) {
        let mut leaf = Vec::new();
        for j in (i+1)..chars.len() {
          leaf.push(chars[j]);
        }
        output[ci].insert(chars[i], vec![Node::Leaf(leaf)]);
        break;
      }
      let mut child_index = None;
      let mut leaf_overlap = None;

      'inner: for (ni, node) in output[ci].get(&chars[i]).unwrap().into_iter().enumerate() {
        match node {
          Node::Child(cni) => {
            child_index = Some(*cni);
            if i + 1 >= chars.len() {
              continue; // expecting a nil leaf
            }
            if !output[*cni].contains_key(&chars[i + 1]) {
              continue; // expecting a diff child
            }
            ci = *cni; // need to check this child
            continue 'outer;
          },
          Node::Leaf(leaf) => {
            if leaf.len() == 0 && i + 1 >= chars.len() {
              break 'outer; // found a nil leaf we were expecting
            }
            if leaf.len() == 0 || i + 1 >= chars.len() || leaf[0] != chars[i + 1] {
              continue; // no overlap
            }
            for break_i in 1..leaf.len() {
              if i + 1 + break_i < chars.len() && leaf[break_i] == chars[i + 1 + break_i] {
                continue;
              }
              // some overlap, need to create new child
              let l_leaf: Vec<char> = leaf[(break_i - 1)..].into_iter().map(|ch| *ch).collect();
              leaf_overlap = Some((ni, break_i, l_leaf));
              continue 'inner;
            }
            if leaf.len() + i + 1 == chars.len() {
              // identical, so no change needed 
              break 'outer;
            } else {
              // Chars is an extension of leaf not a break
              leaf_overlap = Some((ni, leaf.len(), vec![leaf[leaf.len() - 1]]));
              continue;
            }
          }

        }
      }

      if leaf_overlap.is_none() {
        // base case new leaf, no connection to anything else in tree
        output[ci].get_mut(&chars[i]).unwrap().push(
          Node::Leaf(chars[i + 1..].into_iter().map(|ch| *ch).collect())
        );
        break;
      }

      // create new child at end so we can re-use existing children if present
      let (leaf_ni, break_i, mut l_leaf) = leaf_overlap.unwrap();
      // println!("{:?} {:?} {:?} {:?}", leaf_ni, l_leaf, break_i, i);

      // First pair of chars where they differ
      let c_leaf_char = chars[i + break_i];
      let l_leaf_char = l_leaf[0];

      l_leaf.remove(0);

      // Leaf nodes representing difference
      let c_leaf = chars[(i + 1 + break_i)..].into_iter().map(|ch| *ch).collect();
      // println!("{:?}", c_leaf);
      // println!("{:?}", l_leaf);
      let l_leaf = Node::Leaf(l_leaf);
      let c_leaf = Node::Leaf(c_leaf);
      
      // remove leaf as no longer needed
      output[ci].get_mut(&chars[i]).unwrap().remove(leaf_ni);

      // Child of current node we are going to add these leafs under
      let n_child_i = child_index.unwrap_or_else(|| {
        let new_child_node = Node::Child(output.len());
        output[ci].get_mut(&chars[i]).unwrap().push(new_child_node);
        output.push(BTreeMap::new());
        output.len() - 1
      });

      if output[n_child_i].contains_key(&chars[i + 1]) {
        panic!("should not be possible");
      }

      // Now get the last grand child, uner the child which can take the leafs
      let mut grand_child_i = n_child_i;
      for matching_i in 0..(break_i - 1) {
        let char_i = i + matching_i + 1;
        if output[grand_child_i].contains_key(&chars[char_i]) {
          panic!("Unsure if this should be possible, handle later");
        }
        // println!(" - {} {}", grand_child_i, chars[char_i]);

        let new_child_node = Node::Child(output.len());
        output[grand_child_i].insert(chars[char_i], vec![new_child_node]);
        grand_child_i = output.len();
        output.push(BTreeMap::new());
      }

      if l_leaf_char == c_leaf_char {
        // Can happen when the break occoured not due to difference, but extension
        output[grand_child_i].insert(c_leaf_char, vec![c_leaf, l_leaf]);
      } else {
        output[grand_child_i].insert(c_leaf_char, vec![c_leaf]);
        output[grand_child_i].insert(l_leaf_char, vec![l_leaf]);
      }
      
      break;
    }

  }

  // println!("{:?}", output);

  output
}

fn part_2(towels: Vec<String>, patterns: Vec<String>) -> u64 {
  let mut possible = 0;
  let tree = make_parse_tree(&towels);

  for t in towels {
    if matches(&tree, t.as_str()) == 0 {
      panic!("Should be able to match {}", t)
    }
  }

  for p in patterns {
    let count = matches(&tree, p.as_str()) as u64;
    println!("{} -> {}", p, count);
    possible += count;
  }

  possible
}