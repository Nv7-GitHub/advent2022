use std::collections::HashMap;

enum MonkeyOp {
  Number(i64),
  Add(usize, usize),
  Mul(usize, usize),
  Sub(usize, usize),
  Div(usize, usize),
  UnparsedOp(String, char, String),
}

struct Monkey {
  name: String,
  op: MonkeyOp,
}


impl Monkey {
  fn value(&self, monkeys: &Vec<Monkey>) -> i64 {
    match self.op {
      MonkeyOp::Number(n) => n,
      MonkeyOp::Add(left, right) => monkeys[left].value(monkeys) + monkeys[right].value(monkeys),
      MonkeyOp::Mul(left, right) => monkeys[left].value(monkeys) * monkeys[right].value(monkeys),
      MonkeyOp::Sub(left, right) => monkeys[left].value(monkeys) - monkeys[right].value(monkeys),
      MonkeyOp::Div(left, right) => monkeys[left].value(monkeys) / monkeys[right].value(monkeys),
      MonkeyOp::UnparsedOp(_, _, _) => panic!("unparsed op")
    }
  }

  fn algebra(&self, monkeys: &Vec<Monkey>) -> String {
    if self.name == "root" {
      let (left, right) = match self.op {
        MonkeyOp::Add(left, right) => (left, right),
        MonkeyOp::Div(left, right) => (left, right),
        MonkeyOp::Mul(left, right) => (left, right),
        MonkeyOp::Sub(left, right) => (left, right),
        _ => panic!("root is not an algebra op")
      };
      return format!("{} = {}", monkeys[left].algebra(monkeys), monkeys[right].algebra(monkeys));
    }
    if self.name == "humn" {
      return "x".to_string();
    }

    match self.op {
      MonkeyOp::Number(n) => n.to_string(),
      MonkeyOp::Add(left, right) => format!("({} + {})", monkeys[left].algebra(monkeys), monkeys[right].algebra(monkeys)),
      MonkeyOp::Mul(left, right) => format!("({} * {})", monkeys[left].algebra(monkeys), monkeys[right].algebra(monkeys)),
      MonkeyOp::Sub(left, right) => format!("({} - {})", monkeys[left].algebra(monkeys), monkeys[right].algebra(monkeys)),
      MonkeyOp::Div(left, right) => format!("({} / {})", monkeys[left].algebra(monkeys), monkeys[right].algebra(monkeys)),
      MonkeyOp::UnparsedOp(_, _, _) => panic!("unparsed op")
    }
  }
}

pub fn day21() {
  // Parse
  let inp = include_str!("day21.txt");
  let mut names = HashMap::new();
  let mut monkeys = Vec::new();
  for line in inp.split("\n") {
    // Split
    let parts = line.split(": ").collect::<Vec<_>>();
    let name = parts[0].to_string();

    // Rhs
    let rhs = parts[1].split(" ").collect::<Vec<_>>();
    let op = if rhs.len() == 1 {
      MonkeyOp::Number(rhs[0].parse().unwrap())
    } else {
      MonkeyOp::UnparsedOp(rhs[0].to_string(), rhs[1].chars().nth(0).unwrap(), rhs[2].to_string())
    };

    // Put in
    names.insert(name.clone(), monkeys.len());
    monkeys.push(Monkey { name, op });
  }

  // Parse rhs
  for m in monkeys.iter_mut() {
    match &m.op {
      MonkeyOp::UnparsedOp(left, op, right) => {
        let left = *names.get(left).unwrap();
        let right = *names.get(right).unwrap();
        m.op = match op {
          '+' => MonkeyOp::Add(left, right),
          '*' => MonkeyOp::Mul(left, right),
          '/' => MonkeyOp::Div(left, right),
          '-' => MonkeyOp::Sub(left, right),
          _ => panic!("unknown op: {}", op)
        }
      }
      _ => {}
    }
  }

  // Calculate for root
  let root = *names.get(&"root".to_string()).unwrap();
  println!("Part 1: {}", monkeys[root].value(&monkeys));

  // Calculate algebra equations
  println!("Algebra: {}", monkeys[root].algebra(&monkeys));
  // Copy-paste into mathpapa and solve!
}