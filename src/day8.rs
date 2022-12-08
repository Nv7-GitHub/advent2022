pub struct Grid {
  vals: Vec<Vec<usize>>
}

impl Grid {
  fn get(&self, row: i32, col: i32) -> usize {
    return self.vals[row as usize][col as usize];
  }

  pub fn visible(&self, row: usize, col: usize) -> bool {
    // Go through each offset
    for rowoff in -1..2_i32 {
      'off: for coloff in -1..2_i32 {
        if (rowoff == 0 && coloff == 0) || (rowoff.abs() == 1 && coloff.abs() == 1) { // Ignore diagonals & no change
          continue;
        }

        // Check if visible through this offset
        let mut rowv = row as i32 + rowoff;
        let mut colv = col as i32 + coloff;
        while rowv >= 0 && colv >= 0 && rowv < self.vals.len() as i32 && colv < self.vals[0].len() as i32 {
          if self.get(rowv, colv) >= self.vals[row][col] {
            continue 'off; // Not visible in this direction
          }
          rowv += rowoff;
          colv += coloff;
        }

        // Succeeded!
        return true;
      }
    }
    false
  }

  pub fn viewingscore(&self, row: usize, col: usize) -> usize {
    let mut score = 1;
     // Go through each offset
     for rowoff in -1..2_i32 {
      for coloff in -1..2_i32 {
        if (rowoff == 0 && coloff == 0) || (rowoff.abs() == 1 && coloff.abs() == 1) { // Ignore diagonals & no change
          continue;
        }

        // Count distance
        let mut rowv = row as i32 + rowoff;
        let mut colv = col as i32 + coloff;
        let mut cnt = 0;
        while rowv >= 0 && colv >= 0 && rowv < self.vals.len() as i32 && colv < self.vals[0].len() as i32 {
          cnt += 1;
          if self.get(rowv, colv) >= self.vals[row][col] {
            break; // Reached as far as we can see
          }
          rowv += rowoff;
          colv += coloff;
        }
        score *= cnt; // Update score
      }
    }
    score
  }
}

pub fn day8() {
  let inp = include_str!("day8.txt");
  let lines = inp.split("\n");
  let grid = Grid{
    vals: lines.map(|x| x.chars().map(|v| v.to_string().parse::<usize>().unwrap()).collect::<Vec<usize>>()).collect(), // Scuffed but it works
  };

  let mut p1 = 0;
  let mut p2 = 0;

  for row in 0..grid.vals.len() {
    for col in 0..grid.vals[0].len() {
      // Part 1
      if grid.visible(row, col) {
        p1 += 1;
      }

      // Part 2
      if grid.viewingscore(row, col) > p2 {
        p2 = grid.viewingscore(row, col);
      }
    }
  }
  println!("Part 1: {}", p1);
  println!("Part 2: {}", p2);
}