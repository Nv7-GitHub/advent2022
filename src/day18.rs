const XLEN: usize = 20;
const YLEN: usize = 20;
const ZLEN: usize = 20;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos(i32, i32, i32); // x, y, z

struct Board {
  board: Vec<Vec<Vec<bool>>>,
}

impl Board { 
  // Floodfills areas where there are not voxels
  fn flood(&self, pos: Pos, res: &mut Vec<Pos>, offsets: &Vec<Pos>) {
    // Check if pos is valid
    if pos.0 < 0 || pos.1 < 0 || pos.2 < 0 || pos.0 >= self.board.len() as i32 || pos.1 >= self.board[0].len() as i32 || pos.2 >= self.board[0][0].len() as i32 { // Out of bounds
      return;
    }
    if self.board[pos.0 as usize][pos.1 as usize][pos.2 as usize] == true { // Is a voxel, don't include
      return;
    }
    if res.contains(&pos) { // Already visited
      return;
    }

    // Floodfill
    res.push(pos);
    for v in offsets {
      let newpos = Pos(pos.0 + v.0, pos.1 + v.1, pos.2 + v.2);
      self.flood(newpos, res, offsets);
    }
  }

  fn surfarea(&self, pos: Pos, offsets: &Vec<Pos>) -> usize {
    let mut surfarea = 6;
    for v in offsets {
      let newpos = Pos(pos.0 + v.0, pos.1 + v.1, pos.2 + v.2);
      if newpos.0 < 0 || newpos.1 < 0 || newpos.2 < 0 || newpos.0 >= self.board.len() as i32 || newpos.1 >= self.board[0].len() as i32 || newpos.2 >= self.board[0][0].len() as i32 { // Out of bounds
        continue;
      }
      if self.board[newpos.0 as usize][newpos.1 as usize][newpos.2 as usize] == false { // Not a voxel
        continue;
      }
      surfarea -= 1;
    }
    surfarea
  }

  // Surfarea but it only counts blank voxels if they are in the domain
  fn surfarea_with_domain(&self, pos: Pos, offsets: &Vec<Pos>, domain: &Vec<Pos>) -> usize {
    let mut surfarea = 6;
    for v in offsets {
      let newpos = Pos(pos.0 + v.0, pos.1 + v.1, pos.2 + v.2);
      if newpos.0 < 0 || newpos.1 < 0 || newpos.2 < 0 || newpos.0 >= self.board.len() as i32 || newpos.1 >= self.board[0].len() as i32 || newpos.2 >= self.board[0][0].len() as i32 { // Out of bounds
        continue;
      }
      if self.board[newpos.0 as usize][newpos.1 as usize][newpos.2 as usize] == false && domain.contains(&newpos) { // Not a voxel
        continue;
      }
      surfarea -= 1;
    }
    surfarea
  }
}

pub fn day18() {
  // Make offsets
  let mut offsets = Vec::new();
  for x in -1i32..2 {
    for y in -1i32..2 {
      for z in -1i32..2 {
        if x.abs() + y.abs() + z.abs() == 1 {
          offsets.push(Pos(x, y, z));
        }
      }
    }
  }

  // Parse
  let mut board = Board {
    board: vec![vec![vec![false; ZLEN]; YLEN]; XLEN],
  };
  let inp = include_str!("day18.txt");
  for line in inp.split("\n") {
    let parts = line.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    board.board[parts[0]][parts[1]][parts[2]] = true;
  }

  // Part 2
  // Find all space that is not voxels by floodfilling from bottom right corner (assumes nothing encloses that)
  let mut blankspace = Vec::new();
  board.flood(Pos(XLEN as i32-1, YLEN as i32-1, ZLEN as i32-1), &mut blankspace, &offsets);
  println!("[DEBUG] Space count: {}", blankspace.len());

  // Go through voxels
  let mut p1 = 0;
  let mut p2 = 0;
  for x in 0..XLEN {
    for y in 0..YLEN {
      for z in 0..ZLEN {
        if board.board[x][y][z] {
          let pos = Pos(x as i32, y as i32, z as i32);

          // Part 1
          p1 += board.surfarea(pos, &offsets);

          // Part 2
          p2 += board.surfarea_with_domain(pos, &offsets, &blankspace);
        }
      }
    }
  }
  println!("Part 1: {}", p1);
  println!("Part 2: {}", p2);

  // Find chunks
  /*let mut res = Vec::new();
  loop {
    // Find voxel
    let mut found = false;
    for x in 0..XLEN {
      for y in 0..YLEN {
        for z in 0..ZLEN {
          if board.board[x][y][z] {
            // Found a voxel, floodfill
            let mut vals = Vec::new();
            board.flood(Pos(x as i32, y as i32, z as i32), &mut vals, &offsets, false);

            // Remove from board
            for v in vals.iter() {
              board.board[v.pos.0 as usize][v.pos.1 as usize][v.pos.2 as usize] = false;
            }

            // Save
            res.push(vals);
            found = true;
          }
        }
      }
    }
    if !found {
      break
    }
  }
  println!("Chunk count: {}", res.len());*/
}