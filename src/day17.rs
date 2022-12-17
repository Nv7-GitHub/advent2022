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
    [false, false, false, true],
    [false, false, false, true],
    [false, false, false, true],
    [true, true, true, true],
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
const HEIGHT: usize = 100;

struct Board {
  board: [[bool; WIDTH]; HEIGHT],
}

struct Pos(usize, usize);

pub fn day17() {
  let inp = include_str!("day17.txt");
}