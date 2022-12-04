struct Range {
  left: usize,
  right: usize,
}

impl Range {
  fn from(v: &str) -> Self {
    let parts: Vec<&str> = v.split("-").collect();
    Self {
      left: parts[0].parse().unwrap(),
      right: parts[1].parse().unwrap(),
    }
  }

  fn contains(&self, other: &Range) -> bool {
    if self.left <= other.left && self.right >= other.right {
      return true;
    }
    false
  }

  fn overlaps(&self, other: &Range) -> bool {
    if self.right < other.left || self.left > other.right {
      return false;
    }
    true
  }
}

pub fn day4() {
  let inp = include_str!("day4.txt");

  let mut p1: usize = 0;
  let mut p2: usize = 0;
  for line in inp.split("\n") {
    let parts: Vec<&str> = line.split(",").collect();
    let r1 = Range::from(parts[0]);
    let r2 = Range::from(parts[1]);
    if r1.contains(&r2) || r2.contains(&r1) {
      p1 += 1;
    }
    if r1.overlaps(&r2) {
      p2 += 1;
    }
  }
  println!("Part 1: {}", p1);
  println!("Part 2: {}", p2);
}