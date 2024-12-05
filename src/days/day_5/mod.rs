
use crate::days::Part;
use crate::util::read_lines;

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
struct Order {
   pre: u32,
   post: u32
}

pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
   let Ok(lines) = read_lines(file_name) else {
      return Err("Failed to read lines");
   };

   let mut orderings = Vec::new();
   let mut updates = Vec::new();
   let mut parsing_order = true;


   for line in lines {
      let l = line.expect("Input");

      if l.len() == 0 {
         parsing_order = false;
         continue;
      }
      
      if parsing_order {
         let (l, r) = l.split_once('|').expect("Failed to split");
         let l = l.parse::<u32>().expect("failed to parse l");
         let r = r.parse::<u32>().expect("failed to parse r");

         orderings.push(Order {
            pre: l,
            post: r
         });
      } else {
         let nums = l.split(",")
            .into_iter()
            .map(|n| n.parse::<u32>().expect("failed to parse num"))
            .collect();

         updates.push(nums);
      }
   }

   println!("{:?}", orderings);
   println!("{:?}", updates);

   match part {
      Part::P1 => Ok(part_1(orderings, updates)),
      Part::P2 => Ok(part_2(orderings, updates)),
   }
}

fn part_1(orderings: Vec<Order>, updates: Vec<Vec<u32>>) -> u64 {
   let mut sum = 0;
   let mut deny_map = BTreeMap::new();

   for ordering in orderings {
      if !deny_map.contains_key(&ordering.pre) {
         deny_map.insert(ordering.pre, BTreeSet::new());
      }
      (deny_map.get_mut(&ordering.pre).expect("failed")).insert(ordering.post);
   }

   for mut update in updates {
      if is_corrrect(&deny_map, &mut update, false) {
         sum += update[update.len() / 2];
      }
   }

   sum.into()
}

fn is_corrrect(
   orderings: &BTreeMap<u32, BTreeSet<u32>>, 
   update: &mut Vec<u32>,
   repair: bool
) -> bool {
   let mut seen = BTreeSet::new();
   let mut was_valid = true;

   let len = update.len();
   let mut i = 0;

   loop {
      if i == len {
         break;
      }

      let page = update[i];

      if !orderings.contains_key(&page) {
         seen.insert(page);
         i += 1;
         continue;
      }

      for deny in orderings.get(&page).expect("oof") {
         if seen.contains(deny) {
            // println!("invalid {} not before {}", deny, page);

            was_valid = false;

            if !repair {
               break;
            }

            
            for j in 0..i {
               if update[j] != *deny {
                  continue;
               }

               for k in (j..i).rev() {
                  update[k + 1] = update[k];
               }
               update[j] = page;
               i = j;
               // println!("re-ordered: {:?}", update);
               break;
            }
         }
      }

      seen.insert(page);
      i += 1;
   }

   was_valid
}


fn part_2(orderings: Vec<Order>, updates: Vec<Vec<u32>>) -> u64 {
   let mut sum = 0;
   let mut deny_map = BTreeMap::new();

   for ordering in orderings {
      if !deny_map.contains_key(&ordering.pre) {
         deny_map.insert(ordering.pre, BTreeSet::new());
      }
      (deny_map.get_mut(&ordering.pre).expect("failed")).insert(ordering.post);
   }

   for mut update in updates {
      if !is_corrrect(&deny_map, &mut update, true) {
         sum += update[update.len() / 2];
      }
   }

   sum.into()
}
