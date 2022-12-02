pub fn day1() {
  // Parse
  let inp: Vec<&str> = include_str!("day1.txt").split("\n\n").collect();
  let mut chunks: Vec<usize> = vec![0; inp.len()];
  for (i, chk) in inp.into_iter().enumerate() {
    for c in chk.split("\n") {
      chunks[i] += c.parse::<usize>().unwrap();
    }
  }

  // Part 1
  println!("Part 1: {}", chunks.iter().max().unwrap());

  // Part 2
  chunks.sort();
  let mut out = 0;
  for (i, v) in chunks.into_iter().rev().enumerate() {
    if i < 3 { // First 3
      out += v;
    }
  }
  println!("Part 2: {}", out);
}
