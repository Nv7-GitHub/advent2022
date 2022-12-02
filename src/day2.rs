enum Result {
  Draw,
  Win,
  Lose
}

impl Result {
  fn score(&self) -> usize {
    match self {
      Self::Draw => 3,
      Self::Win => 6,
      Self::Lose => 0,
    }
  }
  fn parse(val: char) -> Self {
    match val {
      'X' => Self::Lose,
      'Y' => Self::Draw,
      'Z' => Self::Win,
      _ => panic!("invalid result: {}", val),
    }
  }
}

#[derive(Clone, Copy)]
enum Move {
  Rock,
  Paper,
  Scissors
}

impl Move {
  // Parsing
  fn from_opp(opp: char) -> Self {
    match opp {
      'A' => Self::Rock,
      'B' => Self::Paper,
      'C' => Self::Scissors,
      _ => panic!("invalid opp val: {}", opp)
    }
  }
  fn from_self(opp: char) -> Self {
    match opp {
      'X' => Self::Rock,
      'Y' => Self::Paper,
      'Z' => Self::Scissors,
      _ => panic!("invalid self val: {}", opp)
    }
  }

  // Scoring
  fn score(&self) -> usize {
    match self {
      Self::Rock => 1,
      Self::Paper => 2,
      Self::Scissors => 3,
    }
  }

  // Calc win
  pub fn wins_against(&self, b: Self) -> Result {
    let score_1 = self.score();
    let score_2 = b.score();
    if score_2 == 1 && score_1 == 3 {
      return Result::Lose;
    }
    if (score_1 == 1 && score_2 == 3) || (score_1 > score_2) {
      return Result::Win;
    }
    if score_1 == score_2 {
      return Result::Draw;
    }
    Result::Lose
  }

  // Create based on result
  pub fn cause_result(&self, res: Result) -> Self {
    match res {
      Result::Lose => match self {
        Self::Rock => Self::Scissors,
        Self::Paper => Self::Rock,
        Self::Scissors => Self::Paper,
      },
      Result::Draw => *self,
      Result::Win => match self {
        Self::Rock => Self::Paper,
        Self::Paper => Self::Scissors,
        Self::Scissors => Self::Rock,
      }
    }
  }
}

pub fn day2() {
  let inp = include_str!("day2.txt");
  let mut score: usize = 0;
  let mut p2_score: usize = 0;
  for line in inp.split("\n") {
    let opp = Move::from_opp(line.chars().nth(0).unwrap());
    let me = Move::from_self(line.chars().nth(2).unwrap());
    score += me.score() + me.wins_against(opp).score();

    // Part 2
    let res = Result::parse(line.chars().nth(2).unwrap());
    let me = opp.cause_result(res);
    p2_score += me.score() + me.wins_against(opp).score();
  }
  println!("Part 1: {}", score);
  println!("Part 2: {}", p2_score);
}