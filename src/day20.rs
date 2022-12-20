const KEY: i64 = 811589153;
const PART2: bool = true;

#[derive(Debug, Clone, Copy)]
struct Item {
  value: i64,
  original: i64,
}

fn mix(vals: &mut Vec<Item>) {
  for i in 0..vals.len() {
    let ind = vals.iter().position(|x| x.original == i as i64).unwrap();
    if vals[ind].value == 0 {
      continue;
    }

    // Wrap around
    let v = vals.remove(ind);
    let mut newind = ind as i64 + v.value; // raw new index

    // bring it above 0
    if newind < 0 { // One big step to bring it above 0
      let diff = (-newind)/vals.len() as i64;
      newind += diff * vals.len() as i64;
    }
    while newind < 0 { // Small step to bring it above 0
      newind += vals.len() as i64;
    }

    // Wrap around
    newind %= vals.len() as i64;
    if newind == 0 {
      newind += vals.len() as i64;
    }

    // Move
    vals.insert(newind as usize, v);
  }
}

pub fn day20() {
  // Parse
  let inp = include_str!("day20.txt");
  let mut vals = Vec::new();
  for line in inp.split("\n") {
    let item = Item {
      value: line.parse().unwrap(),
      original: vals.len() as i64,
    };
    vals.push(item);
  }

  // Apply decryption key if needed
  if PART2 {
    for v in vals.iter_mut() {
      v.value *= KEY;
    }
  }

  // Mix
  if PART2 {
    for i in 0..10 {
      println!("Mix {}", i);
      mix(&mut vals);
    }
  } else {
    mix(&mut vals);
  }

  // Get vals
  let start = vals.iter().position(|x| x.value == 0).unwrap();
  let v1 = vals[(start + 1000) % vals.len()].value;
  let v2 = vals[(start + 2000) % vals.len()].value;
  let v3 = vals[(start + 3000) % vals.len()].value;
  println!("Result: {}", v1 + v2 + v3); // Change PART_2 to true for part 2 result
}