#[derive(Debug)]
struct FS {
  pub dirs: Vec<Dir>,
}

#[derive(Debug)]
struct Dir {
  path: Path,
  subdirs: Vec<Path>,
  files: Vec<File>,
}

impl FS {
  pub fn get_dir_mut(&mut self, path: &Path) -> Option<&mut Dir> {
    for dir in self.dirs.iter_mut() {
      if dir.path == *path {
        return Some(dir);
      }
    }
    None
  }
  pub fn get_dir(&self, path: &Path) -> Option<&Dir> {
    for dir in self.dirs.iter() {
      if dir.path == *path {
        return Some(dir);
      }
    }
    None
  }

  pub fn get_dir_size(&self, path: &Path) -> usize {
    let mut size = 0;
    let dir = self.get_dir(path).unwrap();
    for file in dir.files.iter() {
      size += file.size;
    }
    for subdir in dir.subdirs.iter() {
      size += self.get_dir_size(subdir);
    }
    size
  }
}

#[derive(PartialEq, Clone, Debug)]
struct Path {
  vals: Vec<String>,
}

impl Path {
  pub fn pop(&mut self) {
    self.vals.pop();
  }

  pub fn push(&mut self, val: String) {
    self.vals.push(val);
  }
}

#[derive(Debug)]
struct File {
  name: String,
  size: usize,
}

pub fn day7() {
  let inp = include_str!("day7.txt");
  let mut root = FS{dirs: vec![Dir{path: Path{vals: Vec::new()}, subdirs: Vec::new(), files: Vec::new()}]};
  let mut curr = Path{vals: Vec::new()};
  let lines: Vec<&str> = inp.lines().collect();
  
  // Run commands
  for (i, line) in inp.lines().enumerate() {
    if line.starts_with("$ ") { // Command
      let parts: Vec<&str> = line.split(" ").collect();
      let cmd = parts[1];
      match cmd {
        // Cd
        "cd" => {
          let arg = parts[2];
          match arg {
            "/" => {
              curr = Path{vals: Vec::new()};
            },
            ".." => {
              curr.pop();
            },
            _ => {
              curr.push(arg.to_string());
            }
          }
        },
        // Ls
        "ls" => {
          for line in lines[i+1..].iter() {
            if line.starts_with("$ ") {
              break;
            }
            let parts: Vec<&str> = line.split(" ").collect();
            if parts[0] == "dir" { // Directory
              // Make path
              let mut path = curr.clone();
              path.push(parts[1].to_string());

              // Add to current dir
              let currdir = root.get_dir_mut(&curr).unwrap();
              currdir.subdirs.push(path.clone());

              // Add to root
              root.dirs.push(Dir{path, subdirs: Vec::new(), files: Vec::new()});
            } else { // File
              let file = File {
                size: parts[0].parse().unwrap(),
                name: parts[1].to_string(),
              };
              let currdir = root.get_dir_mut(&curr).unwrap();
              currdir.files.push(file);
            }
          }
        }
        _ => {}
      }
    }
  }

  let mut p1 = 0; // Part 1
  let min_required = 30000000 - (70000000 - root.get_dir_size(&Path{vals: Vec::new()})); // Part 2
  let mut p2 = usize::MAX; // Part 2
  for dir in root.dirs.iter() {
    let size = root.get_dir_size(&dir.path);

    // Part 1
    if size <= 100000 {
      p1 += size;
    }

    // Part 2
    if size >= min_required && size < p2 {
      p2 = size;
    }
  }
  println!("Part 1: {}", p1);
  println!("Part 2: {}", p2);
}