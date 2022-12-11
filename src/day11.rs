use num_bigint::BigUint;

const PART_1: bool = true; // NOTE: Wasn't able to get part 2 working

struct Monkey {
  items: Vec<BigUint>, // Worry level
  op: WorryOp,
  test: u64, // Divisible by
  ontrue: usize, // Index of next monkey
  onfalse: usize, // Index of next monkey

  // Part 1
  inspections: u64,
}

impl Monkey {
  fn inspect(&mut self) {
    self.inspections += 1;
  }
}

#[derive(Debug)]
enum Operation {
  Multiply,
  Add,
}

#[derive(Debug)]
enum Value {
  Number(u64),
  Old,
}

#[derive(Debug)]
struct WorryOp {
  op: Operation,
  value: Value,
}

impl WorryOp {
  fn apply(&self, old: BigUint) -> BigUint {
    match self.op {
      Operation::Multiply => {
        match self.value {
          Value::Number(n) => old * n,
          Value::Old => old.pow(2),
        }
      },
      Operation::Add => {
        match self.value {
          Value::Number(n) => old + n,
          Value::Old => old * 2 as u64,
        }
      },
    }
  }
}

pub fn day11() {
  let mut monkeys = Vec::<Monkey>::new();
  let inp = include_str!("day11.txt").split("\n\n");

  // Parse
  for monkey in inp {
    // Make monkey
    let parts = monkey.split("\n").collect::<Vec<_>>();
    let mut monkey = Monkey{
      items: Vec::new(),
      op: WorryOp{
        op: Operation::Multiply,
        value: Value::Number(0),
      },
      test: 0,
      ontrue: 0,
      onfalse: 0,
      inspections: 0,
    };
    
    // Items
    let items = parts[1].split(": ").collect::<Vec<_>>()[1].split(", ").map(|x| x.parse::<BigUint>().unwrap()).collect::<Vec<_>>();
    monkey.items = items;

    // Operation
    let op = parts[2].split("new = ").collect::<Vec<_>>()[1];
    let op = op.split(" ").collect::<Vec<_>>();
    monkey.op = WorryOp{
      op: match op[1] {
        "*" => Operation::Multiply,
        "+" => Operation::Add,
        _ => panic!("Unknown operation: {}", op[1]),
      },
      value: match op[2] {
        "old" => Value::Old,
        _ => Value::Number(op[2].parse::<u64>().unwrap()),
      },
    };

    // Test
    let test = parts[3].split("divisible by ").collect::<Vec<_>>();
    monkey.test = test[1].parse::<u64>().unwrap();

    // On true & false
    monkey.ontrue = parts[4].split("throw to monkey ").collect::<Vec<_>>()[1].parse::<usize>().unwrap();
    monkey.onfalse = parts[5].split("throw to monkey ").collect::<Vec<_>>()[1].parse::<usize>().unwrap();

    // Save
    monkeys.push(monkey);
  }


  // Simulate monkeys
  let reduceval = BigUint::from(monkeys.iter().map(|x| x.test).product::<u64>()); // Part 2
  for rnd in 0..(if PART_1{20}else{10000}) {
    println!("\nRound {}", rnd+1);

    // Simulate round
    for i in 0..monkeys.len() {
      while monkeys[i].items.len() > 0 {
        let mut item = monkeys[i].items.remove(0);

        // 1. Inspect & apply op
        monkeys[i].inspect();
        item = monkeys[i].op.apply(item);

        if PART_1 {
          // 2. Bored & relieved
          item /= BigUint::from(3 as usize);
        } else {
          // 2. Bored & relieved (mod by all vals together)
          item /= &reduceval;
        }

        // 3. Run test
        if &item % monkeys[i].test == BigUint::from(0 as usize) {
          // 4. Ontrue
          let ind = monkeys[i].ontrue;
          monkeys[ind].items.push(item);
        } else {
          // 5. Onfalse
          let ind = monkeys[i].onfalse;
          monkeys[ind].items.push(item);
        }
      }
    }

    // Print
    if PART_1 {
      for m in monkeys.iter() {
        println!("{:?}", m.items);
      }
    }
  }

  println!("\n\n");

  // Part 1
  let mut inspections = monkeys.iter().map(|x| x.inspections).collect::<Vec<_>>();
  inspections.sort_by(|a, b| b.cmp(a));
  println!("Result: {}", inspections[0] * inspections[1]); // make part_1 to true to get part 1
}