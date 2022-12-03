use std::collections::HashSet;

#[derive(Default)]
struct Backpack {
  left: HashSet<u8>,
  right: HashSet<u8>,
}

impl Backpack {
  fn from(val: &str) -> Self {
    let mut b = Backpack::default();
    for (i, char) in val.bytes().enumerate() {
      if i < val.len()/2 {
        b.left.insert(char);
      } else {
        b.right.insert(char);
      }
    }
    return b;
  }

  fn common(&self) -> Vec<u8> {
    let mut common = Vec::new();
    for v in self.left.iter() {
      if self.right.contains(v) {
        common.push(*v);
      }
    }
    return common;
  }
  
  fn set(&self) -> HashSet<u8> {
    return self.left.clone().union(&self.right).copied().collect();
  }
}

fn score(v: u8) -> u8 {
  if 96 < v && v < 173 {
    return v - 96;
  }
  if 64 < v && v < 91 {
    return v - 38;
  }
  panic!("invalid char: {}", v as char)
}

pub fn day3() {
  let inp: Vec<&str> = include_str!("day3.txt").split("\n").collect();

  // Part 1
  let mut p1: usize = 0;
  for line in inp.iter() {
    let b = Backpack::from(line);
    let common = b.common();
    for v in common {
      p1 += score(v) as usize;
    }
  }
  println!("Part 1: {}", p1);

  // Part 2
  let mut p2: usize = 0;
  for i in 0..inp.len()/3 {
    let b1 = Backpack::from(inp[i*3]);
    let b2 = Backpack::from(inp[i*3+1]);
    let b3 = Backpack::from(inp[i*3+2]);
    let set1 = b1.set();
    let set2 = b2.set();
    let set3 = b3.set();
    let mut common = HashSet::new();
    for v in set1.iter() {
      if set2.contains(&v) && set3.contains(&v) {
        common.insert(*v);
      }
    }
    println!("{:?}{:?}{:?}{:?}", set1, set2, set3, common);
    for v in common {
      p2 += score(v) as usize;
    }
  }
  println!("Part 2: {}", p2);
}