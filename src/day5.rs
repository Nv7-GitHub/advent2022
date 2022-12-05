use regex::Regex;

#[derive(Default, Clone)]
struct Cargo {
  stacks: Vec<Vec<char>>,
}

impl Cargo {
  fn top(&self) -> String {
    let mut out = String::new();
    for stack in self.stacks.iter() {
      out += &stack.last().unwrap().to_string();
    }
    out
  }
}

pub fn day5() {
  let inp = include_str!("day5.txt");
  let parts: Vec<&str> = inp.split("\n\n").collect();

  // Parse starting state
  let mut starting: Vec<&str> = parts[0].split("\n").collect();
  starting.reverse();
  starting.remove(0);
  let mut cargo_p1 = Cargo::default();

  for line in starting {
    for i in 0..(line.len()+1)/4 { // Add 1 to line.len() so that we effectively have trailing space after last crate
      if cargo_p1.stacks.len() <= i {
        cargo_p1.stacks.push(Vec::new());
      }
      let lett = line.chars().nth(4*i+1).unwrap();
      if lett != ' ' {
        cargo_p1.stacks[i].push(lett); 
      }
    }
  }
  let mut cargo_p2 = cargo_p1.clone();

  // Run instructions
  let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
  for instr in parts[1].split("\n") {
    let caps = re.captures(instr).unwrap();
    let movecnt = caps[1].parse::<usize>().unwrap();
    let from = caps[2].parse::<usize>().unwrap() - 1;
    let to = caps[3].parse::<usize>().unwrap() - 1;

    // Part 1 instructions
    for _ in 0..movecnt {
      let v = cargo_p1.stacks[from].pop().unwrap();
      cargo_p1.stacks[to].push(v);
    }

    // Part 2 instructions
    let newlen = cargo_p2.stacks[from].len()-movecnt;
    let vals: Vec<char> = cargo_p2.stacks[from].drain(newlen..).collect();
    cargo_p2.stacks[to].append(&mut vals.clone());
  }

  // Output
  println!("Part 1: {}", cargo_p1.top());
  println!("Part 2: {}", cargo_p2.top());
}