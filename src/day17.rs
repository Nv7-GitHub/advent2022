const ROCKS: [[[bool; 4]; 4]; 5] = [
  [
    [true, true, true, true],
    [false, false, false, false],
    [false, false, false, false],
    [false, false, false, false],
  ],
  [
    [false, true, false, false],
    [true, true, true, false],
    [false, true, false, false],
    [false, false, false, false],
  ],
  [
    [false, false, true, false],
    [false, false, true, false],
    [false, false, true, false],
    [true, true, true, false],
  ],
  [
    [true, false, false, false],
    [true, false, false, false],
    [true, false, false, false],
    [true, false, false, false],
  ],
  [
    [true, true, false, false],
    [true, true, false, false],
    [false, false, false, false],
    [false, false, false, false],
  ],
];
const WIDTH: usize = 7;
const HEIGHT: usize = 15;

struct Board {
  board: [[bool; WIDTH]; HEIGHT],
}

impl Board {
  fn cnt(&self) -> i32 {
    let mut res = 0;
    for i in (0..self.board.len()).rev() {
      let mut filled = false;

      // Check if row has anything
      for v in self.board[i].iter() {
        if *v {
          filled = true;
          break;
        }
      }

      if filled {
        res += 1;
      } else {
        break;
      }
    }

    res
  }

  fn top(&self) -> i32 {
    self.board.len() as i32 - self.cnt() - 1
  }

  fn can_move(&self, vals: &Vec<Pos>) -> bool {
    for val in vals.iter() {
      if val.0 >= self.board.len() as i32 {
        return false;
      }
      if val.0 < 0 {
        return false;
      }
      if val.1 >= self.board[0].len() as i32 {
        return false;
      }
      if val.1 < 0 {
        return false;
      }
      if self.board[val.0 as usize][val.1 as usize] {
        return false;
      }
    }

    true
  }

  fn print(&self, curr: &Vec<Pos>) {
    for (r, row) in self.board.iter().enumerate() {
      for (c, col) in row.iter().enumerate() {
        if curr.contains(&Pos(r as i32, c as i32)) {
          print!("@");
          continue;
        }
        if *col {
          print!("#");
        } else {
          print!(".");
        }
      }
      println!("");
    }
  }
}

fn height(rock: [[bool; 4]; 4]) -> i32 {
  for (i, line) in rock.iter().enumerate() {
    let mut filled = false;
    for v in line.iter() {
      if *v {
        filled = true;
        break;
      }
    }
    if !filled {
      return i as i32;
    }
  }

  rock.len() as i32
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pos(i32, i32);

pub fn day17() {
  // Parse
  let inp = include_str!("day17.txt");
  let mut offs = Vec::new();
  for char in inp.chars() {
    offs.push(match char {
      '>' => Pos(0, 1),
      '<' => Pos(0, -1),
      _ => panic!("invalid direction: {}", char),
    })
  }

  // Make board
  let mut board = Board {
    board: [[false; WIDTH]; HEIGHT],
  };
  
  let mut count = 0;
  let mut pushcnt = 0;

  while count < 4 {
    // Add rock
    let rock = ROCKS[count % ROCKS.len()];
    let off = Pos(board.top() - 3 - height(rock) + 1, 2); // -3 so 3 above, +1 cancels out extra num in rock height
    let mut curr = Vec::new();
    for (r, row) in rock.iter().enumerate() {
      for (c, col) in row.iter().enumerate() {
        if *col {
          curr.push(Pos(off.0 + r as i32, off.1 + c as i32));
        }
      }
    }

    // Simulate

    loop {
      println!("\n\n");
      println!("gravity");
      board.print(&curr);
      println!("\n\n");

      // Push dir
      let dir = offs[pushcnt % offs.len()];
      println!("dir: {:?} {}", dir, pushcnt);
      let mut new = curr.clone();
      for v in new.iter_mut() {
        v.0 += dir.0;
        v.1 += dir.1;
      }
      pushcnt += 1;
      if board.can_move(&new) {
        println!("move");
        curr = new.clone();
      }

      board.print(&curr);

      // Move down
      let mut new = curr.clone();
      for v in new.iter_mut() {
        v.0 += 1;
      }
      if board.can_move(&new) {
        curr = new.clone();
      } else {
        count += 1;
        // Fill in
        for v in curr.iter() {
          board.board[v.0 as usize][v.1 as usize] = true;
        }

        //println!("\n\n");
        //board.print(&Vec::new());
        break;
      }
    }
  }

  println!("Day 17: {}", board.cnt());
}