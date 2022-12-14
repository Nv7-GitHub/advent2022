const WIDTH: usize = 1000;
const HEIGHT: usize = 1000; // Start out at 1000, change it to what program says
const PART2: bool = false;

struct Board {
  cells: [[Cell; WIDTH]; HEIGHT],
}

#[derive(Clone, Copy, Debug)]
struct Cell {
  sand: bool,
  rock: bool,
}

impl Board {
  pub fn get(&self, pos: Pos) -> Cell {
    self.cells[pos.0][pos.1]
  }

  // Returns whether the sand has settled
  pub fn simulate(&mut self, mut sandpos: Pos) -> bool {
    // Check if source is blocked
    if PART2 && self.get(Pos(sandpos.0, sandpos.1)).sand {
      return false;
    }

    loop {
      // Check out of bounds
      if sandpos.0 + 1 >= HEIGHT {
        if !PART2 {
          return false;
        } else {
          self.cells[sandpos.0][sandpos.1].sand = true; // Rest here
          return true;
        }
      }

      // Simulate
      let below = self.get(Pos(sandpos.0 + 1, sandpos.1));
      if below.rock || below.sand {
        // Try moving left
        let left = self.get(Pos(sandpos.0 + 1, sandpos.1 - 1));
        if left.rock || left.sand {
          // Try moving right
          let right = self.get(Pos(sandpos.0 + 1, sandpos.1 + 1));
          if right.rock || right.sand {
            // Rest
            self.cells[sandpos.0][sandpos.1].sand = true;
            return true;
          } else {
            sandpos.0 += 1;
            sandpos.1 += 1;
          }
        } else {
          sandpos.0 += 1;
          sandpos.1 -= 1;
        }
      } else {
        sandpos.0 += 1;
      }
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct Pos(usize, usize); // row, col

pub fn day14() {
  let mut board = Board {
    cells: [[Cell { sand: false, rock: false }; WIDTH]; HEIGHT],
  };
  let mut maxy = 0;

  // Parse
  let inp = include_str!("day14.txt");
  for line in inp.split("\n") {
    let posvals: Vec<_> = line.split(" -> ").map(|x| {
      let vals: Vec<_> = x.split(",").collect();
      Pos(vals[1].parse().unwrap(), vals[0].parse().unwrap())
    }).collect();

    // Put in board
    for (i, _) in posvals.iter().enumerate() {
      if i == 0 {
        continue
      }

      let mut prev = posvals[i - 1];
      let mut curr = posvals[i];

      if prev.0 == curr.0 {
        if prev.1 > curr.1 {
          let buff = curr.1;
          curr.1 = prev.1;
          prev.1 = buff;
        }
        for i in prev.1..(curr.1 + 1) {
          board.cells[prev.0][i].rock = true;
          if prev.0 > maxy {
            maxy = prev.0;
          }
        }
      } else {
        if prev.0 > curr.0 {
          let buff = curr.0;
          curr.0 = prev.0;
          prev.0 = buff;
        }
        for i in prev.0..(curr.0+1) {
          board.cells[i][prev.1].rock = true;
          if i > maxy {
            maxy = prev.0;
          }
        }
      }
    }
  }

  if HEIGHT != maxy + 2 {
    panic!("Change HEIGHT to {}", maxy + 2);
  }

  // Simulate
  let mut result = 0;
  while board.simulate(Pos(0, 500)) {
    result += 1;
  }
  println!("Result: {}", result); // Change PART2 to true for part 2
}