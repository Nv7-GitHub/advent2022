#[derive(Debug, Clone, PartialEq)]
enum AST {
  Number(i32),
  List(Vec<AST>)
}

struct Source {
  src: Vec<char>,
  pos: usize
}

impl Source {
  fn peek(&self) -> char {
    return self.src[self.pos];
  }

  fn next(&mut self) -> char {
    let val = self.src[self.pos];
    self.pos += 1;
    return val;
  }

  fn from(v: &str) -> Self {
    Self {
      src: v.chars().collect(),
      pos: 0,
    }
  }
}

fn parse(src: &mut Source) -> AST {
  let char = src.peek();
  match char {
    '[' => {
      src.next();
      let mut list = Vec::new();
      loop {
        let char = src.peek();
        match char {
          ']' => {
            src.next();
            break;
          }
          ',' => {
            src.next();
            continue;
          }
          _ => {
            list.push(parse(src));
          }
        }
      }

      AST::List(list)
    }
    _ => {
      // Number
      let mut val = String::new();
      loop {
        let char = src.peek();
        match char {
          ',' | ']' => {
            break;
          }
          _ => {
            val.push(char);
            src.next();
          }
        }
      }

      AST::Number(val.parse().unwrap())
    }
  }
}

#[derive(Debug)]
enum Comparison {
  Good,
  Bad,
  Continue,
}

fn compare(left: AST, right: AST) -> Comparison {
  match (left, right) {
    (AST::Number(l), AST::Number(r)) => {
      if l < r {
        Comparison::Good
      } else if l > r {
        Comparison::Bad
      } else {
        Comparison::Continue
      }
    }
    (AST::List(l), AST::List(r)) => {
      let mut li = 0;
      let mut ri = 0;
      loop {
        if li >= l.len() && ri >= r.len() {
          return Comparison::Continue;
        }
        if li >= l.len() {
          return Comparison::Good;
        }
        if ri >= r.len() {
          return Comparison::Bad;
        }
        match compare(l[li].clone(), r[ri].clone()) {
          Comparison::Good => {
            return Comparison::Good;
          }
          Comparison::Bad => {
            return Comparison::Bad;
          }
          Comparison::Continue => {}
        }
        li += 1;
        ri += 1;
      }
    }
    (AST::Number(l), AST::List(r)) => {
      return compare(AST::List(vec![AST::Number(l)]), AST::List(r));
    }
    (AST::List(l), AST::Number(r)) => {
      return compare(AST::List(l), AST::List(vec![AST::Number(r)]));
    }
  }
}

pub fn day13() {
  let inp = include_str!("day13.txt");

  let mut p1 = 0;
  let mut p2_vals = Vec::new();
  for (i, chunk) in inp.split("\n\n").enumerate() {
    let parts = chunk.split("\n").collect::<Vec<_>>();
    let left = parse(&mut Source::from(parts[0]));
    let right = parse(&mut Source::from(parts[1]));
    p2_vals.push(left.clone());
    p2_vals.push(right.clone());
    match compare(left, right) {
      Comparison::Good => {
        p1 += i + 1;
      }
      _ => {}
    }
  }
  println!("Part 1: {}", p1);

  // Part 2
  let divider_1 = AST::List(vec![AST::List(vec![AST::Number(2)])]);
  let divider_2 = AST::List(vec![AST::List(vec![AST::Number(6)])]);

  p2_vals.push(divider_1.clone());
  p2_vals.push(divider_2.clone());

  p2_vals.sort_by(|a, b| {
    match compare(a.clone(), b.clone()) {
      Comparison::Good => std::cmp::Ordering::Less,
      Comparison::Bad => std::cmp::Ordering::Greater,
      Comparison::Continue => std::cmp::Ordering::Equal,
    }

  });
  let pos_1 = p2_vals.iter().position(|v| v.clone() == divider_1).unwrap();
  let pos_2 = p2_vals.iter().position(|v| v.clone() == divider_2).unwrap();

  println!("Part 2: {}", (pos_1+1)*(pos_2+1));
}