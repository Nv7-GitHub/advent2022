struct Board {
  cells: Vec<Vec<u8>>, // byte representation, can use to compare heights too
}

struct Offset(i32, i32); // Row, col

const OFFSETS: [Offset; 4] = [
  Offset(1, 0),
  Offset(0, 1),
  Offset(-1, 0),
  Offset(0, -1),
];

impl Board {
  fn can_go(&self, currpos: Pos, offrow: i32, offcol: i32) -> Option<Pos> {
    let newrw = currpos.0 as i32 + offrow;
    if newrw < 0 || newrw >= self.cells.len() as i32 {
      return None;
    }
    let newcl = currpos.1 as i32 + offcol;
    if newcl < 0 || newcl >= self.cells[newrw as usize].len() as i32 {
      return None;
    }

    let newpos = Pos(newrw as usize, newcl as usize);

    // Figure out new
    let mut new = self.cells[newpos.0][newpos.1] as i32;
    if new == 69 { // E
      new = 122; // E is actually z in terms of height
    }

    if new - self.cells[currpos.0][currpos.1] as i32 > 1 {
      return None;
    }

    Some(newpos)
  }

  fn iddfs(&self, maxdepth: usize, curr: &mut Vec<Pos>) -> Option<Vec<Pos>> {
    if maxdepth == 0 {
      return None;
    }

    let currpos = curr[curr.len()-1];

    for offset in OFFSETS {
      if let Some(newpos) = self.can_go(currpos, offset.0, offset.1) {
        // Check if going in circle
        if curr.contains(&newpos) {
          continue;
        }

        // Check if newpos is end
        if self.cells[newpos.0][newpos.1] == b'E' {
          let mut newpath = curr.clone();
          newpath.push(newpos);
          return Some(newpath);
        }
        
        // Go deeper
        curr.push(newpos);
        if let Some(result) = self.iddfs(maxdepth-1, curr) {
          return Some(result); // Success!
        }
        curr.pop();

        // Continue
      }
    }
    None
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pos(usize, usize); // Row, col

pub fn day12() {
  let mut board = Board{
    cells: Vec::new(),
  };
  let mut start = Pos(0, 0);

  // Parse
  let inp = include_str!("day12.txt");
  for line in inp.split("\n") {
    board.cells.push(line.bytes().collect());
    if line.contains("S") {
      let posrow = board.cells.len() - 1;
      let poscol = line.find("S").unwrap();
      start = Pos(posrow, poscol);
      board.cells[posrow][poscol] = b'{'; // after z
    }
  }

  // BFS
  let mut i = 1;
  loop {
    println!("Depth {}", i);
    if let Some(res) = board.iddfs(i, &mut vec![start]) {
      println!("Length: {}", res.len() - 1);
      break;
    }
    i += 1;
  }
}