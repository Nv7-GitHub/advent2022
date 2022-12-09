#[derive(Clone, Copy, Default)]
struct Cell {
  visited: bool,
}

struct Board {
  cells: Vec<Vec<Cell>>,
  head: Pos,
  tail: Vec<Pos>,
}

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
const TAIL_LEN: usize = 9; // make 10 for part 2

impl Board {
  fn move_tail(&mut self) {
    let mut prev = self.head;
    for v in self.tail.iter_mut() {
      let mut rowdiff = prev.0 - v.0;
      let mut coldiff = prev.1 - v.1;
      if rowdiff.abs() <= 1 && coldiff.abs() <= 1 { // Already touching
        return;
      }

      if rowdiff != 0 {
        rowdiff /= rowdiff.abs(); // Make it 1 or -1
      }
      if coldiff != 0 {
        coldiff /= coldiff.abs(); // Make it 1 or -1
      }
      v.0 += rowdiff;
      v.1 += coldiff;

      // Update visited
      prev = *v;
    }

    self.tail_visited();
  }

  fn tail_visited(&mut self) {
    let v = self.tail[self.tail.len()-1];
    self.cells[v.0 as usize][v.1 as usize].visited = true;
  }

  fn print(&self) {
    for (r, row) in self.cells.iter().enumerate() {
      'pt: for (c, cell) in row.iter().enumerate() {
        if r == self.head.0 as usize && c == self.head.1 as usize {
          print!("H");
          continue 'pt;
        }

        for (i, t) in self.tail.iter().enumerate() {
          if r == t.0 as usize && c == t.1 as usize {
            print!("{}", i+1);
            continue 'pt;
          }
        }

        if cell.visited {
          print!("#");
          continue 'pt;
        }

        print!(".");
      }
      println!("");
    }
  }
}

#[derive(Debug, Clone, Copy)]
struct Pos(i32, i32); // (row, col)

pub fn day9() {
  // Init board
  let mut board = Board {
    cells: vec![vec![Cell::default(); WIDTH]; HEIGHT],
    head: Pos(HEIGHT as i32/2, WIDTH as i32/2),
    tail: vec![Pos(HEIGHT as i32/2, WIDTH as i32/2); TAIL_LEN],
  };
  board.tail_visited();

  // Parse
  let inp = include_str!("day9.txt");
  for line in inp.lines() {
    let parts: Vec<&str> = line.split(" ").collect();
    for _ in 0..parts[1].parse::<usize>().unwrap() {
      // Run instruction
      match parts[0].chars().nth(0).unwrap() {
        'R' => board.head.1 += 1,
        'L' => board.head.1 -= 1,
        'U' => board.head.0 -= 1,
        'D' => board.head.0 += 1,
        _ => {},
      }

      // Move tail
      board.move_tail();


      // Looks cool! (turn down dimensions)
      //println!("\n\n\n");
      //board.print(); 
    }
  }

  // Count for part 1
  let mut p1: usize = 0;
  for row in board.cells {
    for cell in row {
      if cell.visited {
        p1 += 1;
      }
    }
  }
  println!("Part 1: {}", p1);
}