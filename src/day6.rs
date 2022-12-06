use std::collections::HashSet;

pub fn get_required_for_unique(unique: usize, inp: &str) -> usize {
  let mut window = String::new();
  for (i, char) in inp.chars().enumerate() {
    if window.len() < unique { // Fill out window
      window += char.to_string().as_str();
      continue;
    }

    // Move window right
    window.remove(0);
    window += char.to_string().as_str();

    // Check if all unique
    let set: HashSet<char> = HashSet::from_iter(window.chars());
    if set.len() == unique {
      return i+1;
    }
  }

  0
}

pub fn day6() {
  let inp = include_str!("day6.txt");
  println!("Part 1: {}", get_required_for_unique(4, inp));
  println!("Part 2: {}", get_required_for_unique(14, inp));
}