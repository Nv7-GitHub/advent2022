const WIDTH: usize = 40;
const HEIGHT: usize = 6;

struct CPU {
  register: i32,
  cycle: i32,

  p1: i32,

  screen: [[bool; WIDTH]; HEIGHT],
}

impl CPU {
  fn next_cycle(&mut self) {
    self.cycle += 1;
    if (self.cycle-20)%40 == 0 { // Part 1
      self.p1 += self.register * self.cycle;
    }
    self.draw_screen(); // Part 2
  }
  
  fn draw_screen(&mut self) {
    let ind = self.cycle % (WIDTH*HEIGHT) as i32 - 1;
    let row = ind / WIDTH as i32;
    let col = ind % WIDTH as i32;

    // Check if character is here
    if (col-self.register).abs() <= 1 { // Within 1 on each side
      self.screen[row as usize][col as usize] = true;
    }
  }

  fn print_screen(&self) {
    for row in self.screen {
      for col in row {
        if col {
          print!("#");
        } else {
          print!(".");
        }
      }
      println!("");
    }
  }
}

pub fn day10() {
  let inp = include_str!("day10.txt");

  // Init cpu
  let mut cpu = CPU{
    register: 1,
    cycle: 0,
    p1: 0,
    screen: [[false; WIDTH]; HEIGHT],
  };

  for line in inp.lines() {
    let parts: Vec<_> = line.split(" ").collect();
    match parts[0] {
      "noop" => cpu.next_cycle(),
      "addx" => {
        let x = parts[1].parse::<i32>().unwrap();
        cpu.next_cycle();
        cpu.next_cycle();
        cpu.register += x;
      }
      _ => {}
    }
  }

  println!("Part 1: {}", cpu.p1);
  println!("Part 2:");
  cpu.print_screen();
}